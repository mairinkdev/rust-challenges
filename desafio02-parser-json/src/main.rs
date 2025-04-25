use json_parser_str::{parse_json, pretty_print_json, JsonValue};

fn main() {
    let raw = r#"
    {
        "nome": "Yuri fodao",
        "idade": 222,
        "interesses": [
            {
                "id": 1,
                "slug": "caguei na calca",
                "topicos": [{"id": 1, "idk": true}]
            }
        ],
        "ativo": true,
        "meta": null
    }
    "#;

    match parse_json(raw) {
        Ok(parsed) => {
            println!("✅ JSON validado com sucesso:\n{}", pretty_print_json(&parsed, 0));

            if let Some(JsonValue::String(nome)) = parsed.query("nome") {
                println!("Nome: {}", nome);
            }

            if let Some(JsonValue::Number(idade)) = parsed.query("idade") {
                println!("Idade: {}", idade);
            }

            if let Some(JsonValue::String(slug)) = parsed.query("interesses.0.slug") {
                println!("Slug: {}", slug);
            }

            if let Some(JsonValue::Boolean(idk)) = parsed.query("interesses.0.topicos.0.idk") {
                println!("IDK: {}", idk);
            }
        }
        Err(e) => {
            println!("❌ Erro ao parsear JSON: {}", e);
        }
    }
}
