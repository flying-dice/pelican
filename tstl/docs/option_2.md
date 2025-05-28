# Option 2: Disable Sanitization of `package` and `require` in `MissionScripting.lua`

### ⚠️ Risks of Enabling `require` and `package` in Multiplayer

Re-enabling `require` and `package` in `MissionScripting.lua` allows mission scripts to load external Lua modules—but
this comes with security risks in multiplayer:

* **Untrusted module loading**: Missions downloaded from servers can `require` arbitrary Lua files, potentially chaining
  code in unintended ways.
* **No user warning**: Players won't be notified when a mission uses `require`, making it easy to run hidden or unsafe
  code.

**Only enable `require`/`package` in trusted environments. For safer distribution, use [option 1](./option_1.md)
instead.**

### Proceed with Caution!!

To use Pelican in the MSE, you will need to remove the sanitization of the `package` and `require` functions in the
`MissionScripting.lua` file.

This file is located in the `Scripts` folder of your DCS World installation, for example:

> ⚠️ Two dashes `--` are used to comment out the code, so you can remove the sanitization of the `package` and
`require`

{@includeCode ./MissionScripting_1.lua}

Once you have modified the `MissionScripting.lua` file, you can use Pelican in your mission scripts without any
restrictions on the `package` and `require` functions.