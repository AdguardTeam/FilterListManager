// Extracts version from Cargo.toml
fun getVersionFromCargoToml(patchVersion: Int): String {
    val cargoTomlFile = File(rootDir, "../../../Cargo.toml")

    if (!cargoTomlFile.exists()) {
        throw GradleException("Cargo.toml file not found at ${cargoTomlFile.absolutePath}")
    }

    val content = cargoTomlFile.readText()
    val versionRegex = Regex("""version\s*=\s*"([^"]+)"""")
    val matchResult = versionRegex.find(content)
        ?: throw GradleException("Version not found in Cargo.toml")

    val baseVersion = matchResult.groupValues[1]
    val resolvedVersion = "$baseVersion.$patchVersion"
    logger.lifecycle("Version for KMP module: $resolvedVersion")
    return resolvedVersion
}

version = getVersionFromCargoToml(patchVersion = 6)
group = "com.adguard.flm"
