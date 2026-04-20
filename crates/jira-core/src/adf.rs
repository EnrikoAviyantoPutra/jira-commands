use serde_json::{json, Value};

/// Convert an ADF (Atlassian Document Format) JSON value to plain text.
pub fn adf_to_text(value: &Value) -> String {
    let mut output = String::new();
    render_node(value, &mut output, 0);
    output.trim_end().to_string()
}

fn render_node(node: &Value, out: &mut String, depth: usize) {
    let node_type = node.get("type").and_then(|v| v.as_str()).unwrap_or("");

    match node_type {
        "doc" => render_children(node, out, depth),
        "paragraph" => {
            render_children(node, out, depth);
            out.push('\n');
        }
        "text" => {
            if let Some(text) = node.get("text").and_then(|v| v.as_str()) {
                out.push_str(text);
            }
        }
        "hardBreak" => {
            out.push('\n');
        }
        "heading" => {
            let level = node
                .get("attrs")
                .and_then(|a| a.get("level"))
                .and_then(|v| v.as_u64())
                .unwrap_or(1);
            let hashes = "#".repeat(level as usize);
            out.push_str(&hashes);
            out.push(' ');
            render_children(node, out, depth);
            out.push('\n');
        }
        "bulletList" => {
            render_list(node, out, depth, false);
        }
        "orderedList" => {
            render_list(node, out, depth, true);
        }
        "listItem" => {
            render_children(node, out, depth);
        }
        "codeBlock" => {
            let language = node
                .get("attrs")
                .and_then(|a| a.get("language"))
                .and_then(|v| v.as_str())
                .unwrap_or("");
            out.push_str("```");
            out.push_str(language);
            out.push('\n');
            render_children(node, out, depth);
            out.push_str("```\n");
        }
        "blockquote" => {
            let mut inner = String::new();
            render_children(node, &mut inner, depth);
            for line in inner.lines() {
                out.push_str("> ");
                out.push_str(line);
                out.push('\n');
            }
        }
        "rule" => {
            out.push_str("---\n");
        }
        "table" => {
            render_table(node, out, depth);
        }
        _ => {
            // Unknown node — try to render children
            render_children(node, out, depth);
        }
    }
}

fn render_children(node: &Value, out: &mut String, depth: usize) {
    if let Some(children) = node.get("content").and_then(|v| v.as_array()) {
        for child in children {
            render_node(child, out, depth);
        }
    }
}

fn render_list(node: &Value, out: &mut String, depth: usize, ordered: bool) {
    let indent = "  ".repeat(depth);
    if let Some(items) = node.get("content").and_then(|v| v.as_array()) {
        for (i, item) in items.iter().enumerate() {
            let bullet = if ordered {
                format!("{}{}. ", indent, i + 1)
            } else {
                format!("{}- ", indent)
            };
            out.push_str(&bullet);
            let mut item_text = String::new();
            render_node(item, &mut item_text, depth + 1);
            out.push_str(item_text.trim());
            out.push('\n');
        }
    }
}

fn render_table(node: &Value, out: &mut String, depth: usize) {
    let indent = "  ".repeat(depth);
    let mut rows: Vec<Vec<String>> = Vec::new();

    if let Some(row_nodes) = node.get("content").and_then(|v| v.as_array()) {
        for row in row_nodes {
            let mut cells: Vec<String> = Vec::new();
            if let Some(cell_nodes) = row.get("content").and_then(|v| v.as_array()) {
                for cell in cell_nodes {
                    let mut cell_text = String::new();
                    render_children(cell, &mut cell_text, depth + 1);
                    let normalized = cell_text
                        .lines()
                        .map(str::trim)
                        .filter(|line| !line.is_empty())
                        .collect::<Vec<_>>()
                        .join(" <br> ");
                    cells.push(normalized);
                }
            }
            if !cells.is_empty() {
                rows.push(cells);
            }
        }
    }

    if rows.is_empty() {
        return;
    }

    let col_count = rows.iter().map(|r| r.len()).max().unwrap_or(0);
    for row in &mut rows {
        while row.len() < col_count {
            row.push(String::new());
        }
    }

    let mut widths = vec![3usize; col_count];
    for row in &rows {
        for (i, cell) in row.iter().enumerate() {
            widths[i] = widths[i].max(cell.len());
        }
    }

    for (idx, row) in rows.iter().enumerate() {
        out.push_str(&indent);
        out.push('|');
        for (i, cell) in row.iter().enumerate() {
            out.push(' ');
            out.push_str(cell);
            let pad = widths[i].saturating_sub(cell.len());
            out.push_str(&" ".repeat(pad));
            out.push(' ');
            out.push('|');
        }
        out.push('\n');

        if idx == 0 {
            out.push_str(&indent);
            out.push('|');
            for width in &widths {
                out.push(' ');
                out.push_str(&"-".repeat(*width));
                out.push(' ');
                out.push('|');
            }
            out.push('\n');
        }
    }
}

fn convert_table_row<'a>(
    node: &'a comrak::nodes::AstNode<'a>,
    out: &mut Vec<Value>,
    is_header_row: bool,
) {
    use comrak::nodes::NodeValue;

    if !matches!(&node.data.borrow().value, NodeValue::TableRow(_)) {
        return;
    }

    for child in node.children() {
        convert_table_cell(child, out, is_header_row);
    }
}

fn convert_table_cell<'a>(
    node: &'a comrak::nodes::AstNode<'a>,
    out: &mut Vec<Value>,
    is_header_row: bool,
) {
    use comrak::nodes::NodeValue;

    let cell_type = match &node.data.borrow().value {
        NodeValue::TableCell => {
            if is_header_row {
                "tableHeader"
            } else {
                "tableCell"
            }
        }
        _ => return,
    };

    let mut content: Vec<Value> = Vec::new();
    for child in node.children() {
        convert_node(child, &mut content);
    }
    if content.is_empty() {
        content.push(json!({ "type": "paragraph", "content": [] }));
    }

    out.push(json!({
        "type": cell_type,
        "content": content,
        "attrs": {}
    }));
}

/// Convert plain text to ADF JSON — each non-empty line becomes a paragraph.
pub fn plain_text_to_adf(text: &str) -> Value {
    let mut content: Vec<Value> = text
        .lines()
        .filter(|l| !l.is_empty())
        .map(|line| {
            json!({
                "type": "paragraph",
                "content": [{ "type": "text", "text": line }]
            })
        })
        .collect();

    if content.is_empty() {
        content.push(json!({ "type": "paragraph", "content": [] }));
    }

    json!({ "version": 1, "type": "doc", "content": content })
}

/// Convert Markdown text to ADF JSON.
pub fn markdown_to_adf(markdown: &str) -> Value {
    use comrak::{parse_document, Arena, Options};

    let arena = Arena::new();
    let options = Options::default();
    let root = parse_document(&arena, markdown, &options);

    let mut content: Vec<Value> = Vec::new();
    convert_node(root, &mut content);

    json!({
        "version": 1,
        "type": "doc",
        "content": content
    })
}

fn convert_node<'a>(node: &'a comrak::nodes::AstNode<'a>, out: &mut Vec<Value>) {
    use comrak::nodes::{ListType, NodeValue};

    match &node.data.borrow().value {
        NodeValue::Document => {
            for child in node.children() {
                convert_node(child, out);
            }
        }
        NodeValue::Table(_) => {
            let mut rows: Vec<Value> = Vec::new();
            for (idx, child) in node.children().enumerate() {
                let mut row_cells: Vec<Value> = Vec::new();
                convert_table_row(child, &mut row_cells, idx == 0);
                if !row_cells.is_empty() {
                    rows.push(json!({
                        "type": "tableRow",
                        "content": row_cells
                    }));
                }
            }
            out.push(json!({
                "type": "table",
                "attrs": {
                    "isNumberColumnEnabled": false,
                    "layout": "default"
                },
                "content": rows
            }));
        }
        NodeValue::Paragraph => {
            let mut inline_content: Vec<Value> = Vec::new();
            for child in node.children() {
                collect_inline(child, &mut inline_content);
            }
            if !inline_content.is_empty() {
                out.push(json!({
                    "type": "paragraph",
                    "content": inline_content
                }));
            }
        }
        NodeValue::Heading(heading) => {
            let mut inline_content: Vec<Value> = Vec::new();
            for child in node.children() {
                collect_inline(child, &mut inline_content);
            }
            out.push(json!({
                "type": "heading",
                "attrs": { "level": heading.level },
                "content": inline_content
            }));
        }
        NodeValue::List(list) => {
            let mut items: Vec<Value> = Vec::new();
            for child in node.children() {
                let mut item_content: Vec<Value> = Vec::new();
                convert_node(child, &mut item_content);
                items.push(json!({
                    "type": "listItem",
                    "content": item_content
                }));
            }
            let list_type = match list.list_type {
                ListType::Ordered => "orderedList",
                ListType::Bullet => "bulletList",
            };
            out.push(json!({
                "type": list_type,
                "content": items
            }));
        }
        NodeValue::Item(_) => {
            // Handled by parent list node
            let mut para_content: Vec<Value> = Vec::new();
            for child in node.children() {
                convert_node(child, &mut para_content);
            }
            out.extend(para_content);
        }
        NodeValue::CodeBlock(code) => {
            let language = code.info.trim().to_string();
            let text = code.literal.trim_end_matches('\n').to_string();
            out.push(json!({
                "type": "codeBlock",
                "attrs": { "language": language },
                "content": [{ "type": "text", "text": text }]
            }));
        }
        NodeValue::BlockQuote => {
            let mut inner: Vec<Value> = Vec::new();
            for child in node.children() {
                convert_node(child, &mut inner);
            }
            out.push(json!({
                "type": "blockquote",
                "content": inner
            }));
        }
        NodeValue::ThematicBreak => {
            out.push(json!({ "type": "rule" }));
        }
        NodeValue::LineBreak | NodeValue::SoftBreak => {
            // handled inline
        }
        _ => {
            // For other block-level nodes, recurse
            let mut child_content: Vec<Value> = Vec::new();
            for child in node.children() {
                convert_node(child, &mut child_content);
            }
            out.extend(child_content);
        }
    }
}

fn collect_inline<'a>(node: &'a comrak::nodes::AstNode<'a>, out: &mut Vec<Value>) {
    use comrak::nodes::NodeValue;

    match &node.data.borrow().value {
        NodeValue::Text(text) => {
            out.push(json!({ "type": "text", "text": text }));
        }
        NodeValue::SoftBreak => {
            out.push(json!({ "type": "text", "text": " " }));
        }
        NodeValue::LineBreak => {
            out.push(json!({ "type": "hardBreak" }));
        }
        NodeValue::Code(code) => {
            out.push(json!({
                "type": "text",
                "text": code.literal,
                "marks": [{ "type": "code" }]
            }));
        }
        NodeValue::Strong => {
            let mut inner: Vec<Value> = Vec::new();
            for child in node.children() {
                collect_inline(child, &mut inner);
            }
            for mut item in inner {
                let marks = item.get("marks").cloned().unwrap_or_else(|| json!([]));
                let mut marks_arr = marks.as_array().cloned().unwrap_or_default();
                marks_arr.push(json!({ "type": "strong" }));
                item["marks"] = json!(marks_arr);
                out.push(item);
            }
        }
        NodeValue::Emph => {
            let mut inner: Vec<Value> = Vec::new();
            for child in node.children() {
                collect_inline(child, &mut inner);
            }
            for mut item in inner {
                let marks = item.get("marks").cloned().unwrap_or_else(|| json!([]));
                let mut marks_arr = marks.as_array().cloned().unwrap_or_default();
                marks_arr.push(json!({ "type": "em" }));
                item["marks"] = json!(marks_arr);
                out.push(item);
            }
        }
        NodeValue::Link(link) => {
            let mut inner: Vec<Value> = Vec::new();
            for child in node.children() {
                collect_inline(child, &mut inner);
            }
            let url = link.url.clone();
            for mut item in inner {
                let marks = item.get("marks").cloned().unwrap_or_else(|| json!([]));
                let mut marks_arr = marks.as_array().cloned().unwrap_or_default();
                marks_arr.push(json!({
                    "type": "link",
                    "attrs": { "href": url }
                }));
                item["marks"] = json!(marks_arr);
                out.push(item);
            }
        }
        _ => {
            for child in node.children() {
                collect_inline(child, out);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adf_to_text_paragraph() {
        let adf = json!({
            "type": "doc",
            "content": [{
                "type": "paragraph",
                "content": [{ "type": "text", "text": "Hello world" }]
            }]
        });
        assert_eq!(adf_to_text(&adf), "Hello world");
    }

    #[test]
    fn test_adf_to_text_heading() {
        let adf = json!({
            "type": "doc",
            "content": [{
                "type": "heading",
                "attrs": { "level": 2 },
                "content": [{ "type": "text", "text": "Title" }]
            }]
        });
        assert!(adf_to_text(&adf).starts_with("## Title"));
    }

    #[test]
    fn test_markdown_to_adf_paragraph() {
        let adf = markdown_to_adf("Hello world");
        assert_eq!(adf["type"], "doc");
        assert_eq!(adf["content"][0]["type"], "paragraph");
    }

    #[test]
    fn test_markdown_to_adf_heading() {
        let adf = markdown_to_adf("# My Heading");
        assert_eq!(adf["content"][0]["type"], "heading");
        assert_eq!(adf["content"][0]["attrs"]["level"], 1);
    }

    #[test]
    fn test_markdown_table_to_adf_table() {
        let adf = markdown_to_adf("| Name | Status |\n| --- | --- |\n| API | Done |");
        assert_eq!(adf["content"][0]["type"], "table");
        assert_eq!(adf["content"][0]["content"][0]["type"], "tableRow");
        assert_eq!(
            adf["content"][0]["content"][0]["content"][0]["type"],
            "tableHeader"
        );
    }

    #[test]
    fn test_adf_table_to_markdown_table() {
        let adf = json!({
            "type": "table",
            "content": [
                {
                    "type": "tableRow",
                    "content": [
                        {"type": "tableHeader", "content": [{"type": "paragraph", "content": [{"type": "text", "text": "Name"}]}]},
                        {"type": "tableHeader", "content": [{"type": "paragraph", "content": [{"type": "text", "text": "Status"}]}]}
                    ]
                },
                {
                    "type": "tableRow",
                    "content": [
                        {"type": "tableCell", "content": [{"type": "paragraph", "content": [{"type": "text", "text": "API"}]}]},
                        {"type": "tableCell", "content": [{"type": "paragraph", "content": [{"type": "text", "text": "Done"}]}]}
                    ]
                }
            ]
        });
        let text = adf_to_text(&adf);
        assert!(text.contains("| Name "));
        assert!(text.contains("| API"));
    }
}
