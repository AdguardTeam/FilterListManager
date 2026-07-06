plugins {
    alias(libs.plugins.kotlin.multiplatform)
    alias(libs.plugins.android.library)
    alias(libs.plugins.wire)
    id("maven-publish")
}

group = rootProject.group
version = rootProject.version

wire {
    kotlin {
    }
    sourcePath {
        srcDir("../../../protobuf")
    }
}

kotlin {
    androidTarget {
        publishLibraryVariants("release")
        compilations.all {
            kotlinOptions {
                jvmTarget = libs.versions.java.get()
            }
        }
    }

    listOf(
        iosX64(),
        iosArm64(),
        iosSimulatorArm64(),
    ).forEach { target ->
        val archDir = when (target.targetName) {
            "iosX64" -> "x86_64-apple-ios"
            "iosSimulatorArm64" -> "aarch64-apple-ios-sim"
            "iosArm64" -> "aarch64-apple-ios"
            else -> throw Exception("Wrong architecture to link static library")
        }
        val libDir = "$rootDir/../../../../../target"
        val rustLibPath = "${libDir}/${archDir}/release"

        target.compilations.getByName("main") {
            val flmNativeInterop by cinterops.creating {
                defFile = file("FlmNative.def")
                extraOpts(
                    "-libraryPath", rustLibPath,
                    "-staticLibrary", "libfilter_list_manager_ffi.a"
                )
            }
        }

        target.compilations.getByName("test") {
            val flmNativeInterop by cinterops.creating {
                defFile = file("FlmNative.def")
                extraOpts(
                    "-libraryPath", rustLibPath,
                    "-staticLibrary", "libfilter_list_manager_ffi.a"
                )
            }
        }

        target.binaries.all {
            linkerOpts("-L${rustLibPath}", "-lfilter_list_manager_ffi")
        }
    }

    sourceSets {
        commonMain.dependencies {
            implementation(libs.kotlin.stdlib.common)
            api(libs.wire.runtime)
        }

        commonTest.dependencies {
            implementation(libs.kotlin.test)
        }

        androidMain.dependencies {
            implementation(libs.kotlin.stdlib.jdk8)
        }
    }

    targets.withType<org.jetbrains.kotlin.gradle.plugin.mpp.KotlinNativeTarget> {
        compilations["main"].kotlinOptions.freeCompilerArgs += listOf(
            "-Xexport-kdoc",
            "-Xexpect-actual-classes",
        )
    }
}

android {
    namespace = "com.adguard.flm"
    compileSdk = libs.versions.android.compile.sdk.get().toInt()

    ndkVersion = libs.versions.android.ndk.get()

    defaultConfig {
        minSdk = libs.versions.android.min.sdk.get().toInt()

        testInstrumentationRunner = "androidx.test.runner.AndroidJUnitRunner"

        externalNativeBuild {
            cmake {
                targets("filter_list_manager_jni")
                arguments("-DANDROID_STL=c++_static", "-DCMAKE_BUILD_TYPE=RelWithDebInfo")
                abiFilters("armeabi-v7a", "arm64-v8a", "x86", "x86_64")
            }
        }
    }

    lint {
        targetSdk = libs.versions.android.compile.sdk.get().toInt()
    }

    compileOptions {
        sourceCompatibility = JavaVersion.toVersion(libs.versions.java.get())
        targetCompatibility = JavaVersion.toVersion(libs.versions.java.get())
    }

    externalNativeBuild {
        cmake {
            path("CMakeLists.txt")
        }
    }

    packaging {
        jniLibs {
            keepDebugSymbols.add("**/*.so")
        }
    }
}

dependencies {
    androidTestImplementation(libs.androidx.test.ext)
    androidTestImplementation(libs.androidx.test.core)
    androidTestImplementation(libs.espresso.core)
}

tasks {
    register<Task>("buildIosSimulatorX64") {
        doLast {
            exec { commandLine("rustup", "target", "add", "x86_64-apple-ios") }
            exec { commandLine("cargo", "build", "--release", "--lib", "--package", "adguard-flm-ffi", "--target", "x86_64-apple-ios", "--no-default-features", "--features", "rusqlite-bundled,rustls-tls") }
        }
    }

    register<Task>("buildIosArm64") {
        doLast {
            exec { commandLine("rustup", "target", "add", "aarch64-apple-ios") }
            exec { commandLine("cargo", "build", "--release", "--lib", "--package", "adguard-flm-ffi", "--target", "aarch64-apple-ios", "--no-default-features", "--features", "rusqlite-bundled,rustls-tls") }
        }
    }

    register<Task>("buildIosSimulatorArm64") {
        doLast {
            exec { commandLine("rustup", "target", "add", "aarch64-apple-ios-sim") }
            exec { commandLine("cargo", "build", "--release", "--lib", "--package", "adguard-flm-ffi", "--target", "aarch64-apple-ios-sim", "--no-default-features", "--features", "rusqlite-bundled,rustls-tls") }
        }
    }

    getByName("cinteropFlmNativeInteropIosX64").dependsOn("buildIosSimulatorX64")
    getByName("iosX64Test").dependsOn("buildIosSimulatorX64")
    getByName("cinteropFlmNativeInteropIosArm64").dependsOn("buildIosArm64")
    getByName("cinteropFlmNativeInteropIosSimulatorArm64").dependsOn("buildIosSimulatorArm64")
    getByName("iosSimulatorArm64Test").dependsOn("buildIosSimulatorArm64")
}

file("publishing.gradle.kts").takeIf { it.exists() }?.let {
    apply(from = it)
}
