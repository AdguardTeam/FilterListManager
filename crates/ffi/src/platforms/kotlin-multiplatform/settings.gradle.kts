pluginManagement {
    repositories {
        maven("https://ak.int.agrd.dev/maven/maven-virtual")
    }
}

buildscript {
    repositories {
        maven("https://ak.int.agrd.dev/maven/maven-virtual")
    }
    dependencies {
        classpath("com.adguard.android.plugin:gradle-kit:2.0.24")
    }
}

apply(plugin = "gradlekit-settings")

rootProject.name = "FilterListManager"

include(":flm")
