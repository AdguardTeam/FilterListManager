pluginManagement {
    repositories {
        mavenCentral()
        google()
        gradlePluginPortal()
    }
}

dependencyResolutionManagement {
    repositories {
        mavenCentral()
        google()
    }
}

plugins {
    id("org.jetbrains.kotlin.multiplatform") version "1.9.24" apply false
    id("com.android.library") version "8.4.0" apply false
    id("com.google.protobuf") version "0.9.5" apply false
}

rootProject.name = "FilterListManager"

include(":lib")
include(":protobuf-codegen")

