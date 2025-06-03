# JSON-RPC Server with Lua Integration

This project provides a fully functional JSON-RPC server implemented in Rust with integration to Lua.
It supports both HTTP and WebSocket endpoints for receiving and responding to JSON-RPC requests.

See https://flying-dice.github.io/pelican/ for more information.

## Testing

The project includes a comprehensive test suite to ensure the functionality of the JSON-RPC server and Lua integration.

Run the tests from the tstl directory using the following command:

```bash
npm install # to install dependencies
npm test # to run unit tests for the DLL utilities, json encoding etc
npm test:e2e # to run end-to-end tests for the JSON-RPC server which stands up a server and sends requests to it using NodeJS
```

## Building

The project can be built using Cargo. Run the following command in the project root directory:

```bash
cargo build
```

This will build the project in debug mode. To build in release mode, run:

```bash
cargo build --release
```

By default the build will produce a DLL compatible with the lua runtime installed on the host.

The build is set up for DCS world currently as such the lua5.1 directory is provided to ensure compatibility with DCS
world.

Run `build-dcs.bat` to build the project for DCS world creating a compatible DLL.

## Contributing

Contributions are welcome! Please submit a pull request or open an issue to discuss potential changes or improvements.

## License

This project is licensed under the MIT License. See the [LICENSE.md](LICENSE.md) file for details.

## Acknowledgements

Special thanks to the contributors of Actix, mlua, and the Rust and Lua communities for their excellent libraries and
support.
