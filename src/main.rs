use nlp::Pipeline;
use nlp::models::ner::NamedEntity;
use nlp::models::dep_parse::DepTriple;

fn main() {
    // read the question from the command line
    println!("Welcome to use the question answering system!");
    println!("Please input your question:");
    let mut question = String::new();
    std::io::stdin().read_line(&mut question).unwrap();
    let pipeline = Pipeline::default();
    let ner = pipeline.get_named_entity_recognizer("en_core_web_sm").unwrap();
    let dp = pipeline.get_dependency_parser("en_core_web_sm").unwrap();

    // Extract named entities from the question
    let named_entities = ner.predict(question);

    // Extract the main subject of the question using dependency parsing
    let dependencies = dp.predict(question);
    let subject = dependencies
        .into_iter()
        .find(|d| d.relation == "nsubj")
        .and_then(|d| named_entities.iter().find(|ne| ne.start == d.head_index))
        .unwrap()
        .text
        .to_lowercase();

    // Look up the answer in a database or knowledge base
    let answer = match subject.as_ref() {
        "president" => "Emmanuel Macron",
        _ => "I don't know",
    };

    println!("Question: {}", question);
    println!("Answer: {}", answer);
}
