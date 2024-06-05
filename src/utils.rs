use dialoguer::Input;

pub fn get_user_input() -> (String, String) {
    let name: String = Input::new()
        .with_prompt("Digite o nome do usuário")
        .interact_text()
        .unwrap();

    let email: String = Input::new()
        .with_prompt("Digite o email do usuário")
        .interact_text()
        .unwrap();

    (name, email)
}
