pub mod environment;
use environment::environment::Environment;
fn main() {
    let mut env = Environment::new();
    env.set("x".to_string(), 42);
    env.set("y".to_string(), 100);

    let mut child_env = Environment::new_with_parent(env);
    child_env.set("x".to_string(), 24);
    child_env.set("z".to_string(), 200);

    println!("x: {:?}", child_env.get("x")); // Output: Some(24)
    println!("y: {:?}", child_env.get("y")); // Output: Some(100)
    println!("z: {:?}", child_env.get("z")); // Output: Some(200)
    println!("x: {:?}", env.get("x")); // Output: Some("42")
}