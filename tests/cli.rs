
use assert_cmd::Command;

#[test]
//a test to invoke the cli with an subcommand 'play' and option --name Marco
fn thirtyfour() {
    let mut cmd = Command::cargo_bin("mini1").unwrap();
    cmd.arg("convert").arg("--num").arg(34);
    cmd.assert().success().stdout("XXXIV\n");

}

#[test]
//a test to invoke the cli with an subcommand 'play' and option --name Bob
fn polo() {
    let mut cmd = Command::cargo_bin("marco_polo").unwrap();
    cmd.arg("play").arg("--name").arg("Bob");
    cmd.assert().success().stdout("Marco\n");
}