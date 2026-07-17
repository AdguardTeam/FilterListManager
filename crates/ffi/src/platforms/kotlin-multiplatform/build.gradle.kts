import com.adguard.gradle.GradleKit

buildscript {
    repositories {
        mavenLocal()
        maven("https://ak.int.agrd.dev/maven/maven-virtual")
    }

    dependencies {
        classpath("com.adguard.android.plugin:gradle-kit:${libs.versions.gradlekit.get()}")
    }
}

plugins {
    alias(libs.plugins.kotlin.multiplatform) apply false
    alias(libs.plugins.android.library) apply false
    alias(libs.plugins.wire) apply false
}

apply(plugin = "gradlekit")
val gradleKit = the<GradleKit>()
gradleKit.loadLocalProperties()

// Resolves the version: the FLM_VERSION env var, else `git describe` over v*
// tags (leading `v` stripped). Same rule as the Rust crates' build scripts.
fun resolveVersion(): String {
    val fromEnv = System.getenv("FLM_VERSION")?.takeIf { it.isNotBlank() }
    val version = (fromEnv ?: gitDescribe()).removePrefix("v")
    logger.lifecycle("Version for KMP module: $version")
    return version
}

fun gitDescribe(): String {
    val process = ProcessBuilder("git", "describe", "--tags", "--match=v*", "--abbrev=0")
        .directory(rootDir)
        .redirectErrorStream(true)
        .start()
    val output = process.inputStream.bufferedReader().use { it.readText() }.trim()
    if (process.waitFor() != 0 || output.isEmpty()) {
        throw GradleException("Cannot resolve version: set FLM_VERSION or create a v* git tag")
    }
    return output
}

version = resolveVersion()
group = "com.adguard.flm"

allprojects {
    repositories {
        mavenLocal {
            content {
                excludeGroupByRegex("org\\.jetbrains\\.kotlin")
                excludeGroupByRegex("org\\.jetbrains\\.kotlinx")
            }
        }
        maven("https://ak.int.agrd.dev/maven/maven-virtual")
    }
}
