import {logger} from "pelican";

logger.debug("This is a debug message");
// 2025-05-28T00:53:29.680377300+01:00 [DEBUG] pelican::logger - This is a debug message

logger.info("This is an info message");
//2025-05-28T00:53:29.680396800+01:00 [INFO] pelican::logger - This is an info message

logger.warn("This is a warning message");
// 2025-05-28T00:53:29.680402400+01:00 [WARN] pelican::logger - This is a warning message

logger.error("This is an error message");
// 2025-05-28T00:53:29.680407200+01:00 [ERROR] pelican::logger - This is an error message

// Create a namespaced logger using the Logger class
const myLogger = new logger.Logger("PELICAN.TEST.INFO");

// Create a namespaced logger using the pelican logger function (Prefer using New keyword in TS)
const _myLogger = logger.Logger("PELICAN.TEST");

myLogger.debug("This is a namespaced debug message");
// 2025-05-28T00:53:29.680531400+01:00 [DEBUG] PELICAN.TEST.INFO - This is a namespaced debug message

_myLogger.info("This is a namespaced info message");
// 2025-05-28T00:53:29.680545100+01:00 [INFO] PELICAN.TEST - This is a namespaced info message

_myLogger.warn("This is a namespaced warning message");
// 2025-05-28T00:53:29.680554900+01:00 [WARN] PELICAN.TEST - This is a namespaced warning message

_myLogger.error("This is a namespaced error message");
// 2025-05-28T00:53:29.680564900+01:00 [ERROR] PELICAN.TEST - This is a namespaced error message