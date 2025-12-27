# This is just a compiled repository, please go to the [original repository](https://github.com/Yoshk4e/sonetto-rs).

Rust can't run on my computer, so I created this repository.
## Thks
-Yoshk4e

## Use

Download latest build from [releases page](https://github.com/Luotianyi-0712/sonetto-rs-build/releases/tag/latest)

Actually, you only need to move the `sdkserver` executable into the `gameserver` folder. The `data/` folder is already included in the build, so no additional setup is required. Ensure your folder structure looks like this:
```text
.
├── sdkserver.exe
├── gameserver.exe
├── Config.toml
└── data/
```
- need to use the [sonetto patch](https://github.com/yoncodes/sonetto-patch) to make the game work with the server
- now open two terminals or command prompts

```bash
    .\sdkserver
```
```bash
    .\gameserver
```
- Login with email. **NOT REGISTER** if the account doesn't exist it will be created automatically
![login image](/images/r99-email.png)


