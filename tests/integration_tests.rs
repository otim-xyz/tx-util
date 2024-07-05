use assert_cmd::Command;

static EIP_1559_UNSIGNED: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/transactions/eip1559_unsigned.json"
));

static EIP_1559_SIGNED: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/transactions/eip1559_signed.json"
));

static EIP_1559_HEX_VALS: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/transactions/eip1559_hex_vals.json"
));

static EIP_7702_UNSIGNED: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/transactions/eip7702_unsigned.json"
));

static EIP_7702_SIGNED: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/transactions/eip7702_signed.json"
));

static EIP_7702_EMPTY_AUTH: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/transactions/eip7702_empty_auth.json"
));

static SIGNER: &str = "34954993d403229ee2e01cf6fa8222224935bc47f9534b0c0ea8054764375501";

#[test]
fn it_runs() {
    let mut cmd = Command::cargo_bin("tx-util").unwrap();
    cmd.assert().success();
}

#[test]
fn it_encodes_1559() {
    let mut cmd = Command::cargo_bin("tx-util").unwrap();
    let assert = cmd
        .arg("encode-tx")
        .args(&["--tx-type", "2"])
        .write_stdin(EIP_1559_SIGNED)
        .assert();
    assert.success().stdout("0x02f8e9018084163ef00185081527974c82f6f594695461ef560fa4d3a3e7332c9bfcec261c11a1b680b844a9059cbb0000000000000000000000005a96834046c1dff63119eb0eed6330fc5007a1d700000000000000000000000000000000000000000000000000000001a1432720f838f7948dfdf61f2eb938b207c228b01a2918b196992abfe1a0000000000000000000000000000000000000000000000000000000000000000301a052ee022a326abb33e6bebab1fa694043371ab41a7a985ea23d48bd78502be87ca05a0f69dc8009a1e449bfbc8b13220bc40337da1325c261afdac1803f26d0e9d5");
}

#[test]
fn it_signs_1559() {
    let mut cmd = Command::cargo_bin("tx-util").unwrap();
    let assert = cmd
        .arg("encode-tx")
        .args(&["--tx-type", "2"])
        .args(&["--signer", SIGNER])
        .write_stdin(EIP_1559_UNSIGNED)
        .assert();
    assert.success().stdout("0x02f8a4010a84163ef00185081527974c82f6f594695461ef560fa4d3a3e7332c9bfcec261c11a1b68080f838f7948dfdf61f2eb938b207c228b01a2918b196992abfe1a0000000000000000000000000000000000000000000000000000000000000000301a0efa0ed9132e900d5dd195698e4a7c14f08dc03c2b3e62b8b9a87b7e08a57c400a00ef4dc89b0c9f4b8e2fdd377e4ed0c3c4c7813a1562c8ada05ebab04925935e7");
}

#[test]
fn it_encodes_1559_hex() {
    let mut cmd = Command::cargo_bin("tx-util").unwrap();
    let assert = cmd
        .arg("encode-tx")
        .args(&["--tx-type", "2"])
        .write_stdin(EIP_1559_HEX_VALS)
        .assert();
    assert.success().stdout("0x02f8a8833018248084163ef00185081527974c830186a094695461ef560fa4d3a3e7332c9bfcec261c11a1b68080f838f7948dfdf61f2eb938b207c228b01a2918b196992abfe1a0000000000000000000000000000000000000000000000000000000000000000301a052ee022a326abb33e6bebab1fa694043371ab41a7a985ea23d48bd78502be87ca05a0f69dc8009a1e449bfbc8b13220bc40337da1325c261afdac1803f26d0e9d5");
}

#[test]
fn it_fails_no_singer_1559() {
    let mut cmd = Command::cargo_bin("tx-util").unwrap();
    let assert = cmd
        .arg("encode-tx")
        .args(&["--tx-type", "2"])
        .write_stdin(EIP_1559_UNSIGNED)
        .assert();
    assert.code(1);
}

#[test]
fn it_encodes_7702() {
    let mut cmd = Command::cargo_bin("tx-util").unwrap();
    let assert = cmd
        .arg("encode-tx")
        .args(&["--tx-type", "4"])
        .write_stdin(EIP_7702_SIGNED)
        .assert();
    assert.success().stdout("0x04f90102018084163ef00185081527974c82f6f594695461ef560fa4d3a3e7332c9bfcec261c11a1b68080f838f7948dfdf61f2eb938b207c228b01a2918b196992abfe1a00000000000000000000000000000000000000000000000000000000000000003f85cf85a0194d571b8bcd11df08f0459009dd1bd664127a431eec001a052ee022a326abb33e6bebab1fa694043371ab41a7a985ea23d48bd78502be87ca05a0f69dc8009a1e449bfbc8b13220bc40337da1325c261afdac1803f26d0e9d501a052ee022a326abb33e6bebab1fa694043371ab41a7a985ea23d48bd78502be87ca05a0f69dc8009a1e449bfbc8b13220bc40337da1325c261afdac1803f26d0e9d5");
}

#[test]
fn it_signs_7702_empty_auth() {
    let mut cmd = Command::cargo_bin("tx-util").unwrap();
    let assert = cmd
        .arg("encode-tx")
        .args(&["--tx-type", "4"])
        .args(&["--signer", SIGNER])
        .write_stdin(EIP_7702_EMPTY_AUTH)
        .assert();
    assert.success().stdout("0x04f86c018084163ef00185081527974c82f6f594695461ef560fa4d3a3e7332c9bfcec261c11a1b68080c0c080a08159b9bdfa233442f45941fa56c0f95c825feadc44a2a0162962e893d93946d6a002225482ae77cccf26f2aa6264f1e34b9815be29678e920c2833f57da2649ebd");
}

#[test]
fn it_signs_7702_and_auths() {
    let mut cmd = Command::cargo_bin("tx-util").unwrap();
    let assert = cmd
        .arg("encode-tx")
        .args(&["--tx-type", "4"])
        .args(&["--signer", SIGNER])
        .args(&["--authorizer", SIGNER])
        .args(&["--authorizer", SIGNER])
        .write_stdin(EIP_7702_UNSIGNED)
        .assert();
    assert.success().stdout("0x04f9015f018084163ef00185081527974c82f6f594695461ef560fa4d3a3e7332c9bfcec261c11a1b68080f838f7948dfdf61f2eb938b207c228b01a2918b196992abfe1a00000000000000000000000000000000000000000000000000000000000000003f8b9f85b0194d571b8bcd11df08f0459009dd1bd664127a431eec10201a0af224f2d45206ef8ed6974fa17337fb148396e2531b14161b04b00d9e63ee34ca03885e8dfcacc288e2519c8be92ad0fb20b78158506fcb0b62829303e48fed13af85a0194d571b8bcd11df08f0459009dd1bd664127a431eec080a050debd048f0d6ab6932a8a7cc5778084fdd8e3d87d51c5b2642942119250ce3ca075c956d12726ff2512ffafe150a06a96fe7664da02d62c0db863c5ff7772135b01a0644c1e935ccdd3a71f6894ab30db8107dad0bbe177c86c447ea2e5900033b3a7a01e01ae276a58089667756d23c9a24c0fdf1d694e3d92de6560222f8dd8b79456");
}
