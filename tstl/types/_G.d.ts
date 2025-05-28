declare let PELICAN: {
    /**
     * The Pelican logger level.
     * This must be set before pelican is imported.
     */
    logger_level?: "debug" | "info" | "warn" | "error" | "off" | undefined;
} | undefined;