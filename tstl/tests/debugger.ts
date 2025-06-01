import { tcpListen, waitIDE } from "emmy_core";

print("Starting Emmy Core Debugger...");
tcpListen("localhost", 9966);

print("Debugger is listening on localhost:9966. Waiting for IDE connection...");
waitIDE();
