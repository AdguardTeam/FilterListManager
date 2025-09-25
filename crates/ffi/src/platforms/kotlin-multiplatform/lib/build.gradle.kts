plugins {
    id("org.jetbrains.kotlin.multiplatform")
    id("com.android.library")
    id("maven-publish")
}

kotlin {
    androidTarget {
        publishLibraryVariants("release", "debug")
        compilations.all {
            kotlinOptions {
                jvmTarget = "17"
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
        commonMain {
            kotlin.srcDir(project
                .files("${rootDir}/protobuf-codegen/build/generated/sources/proto/main/pbandk")
                .builtBy(":protobuf-codegen:generateProto")
            )

            dependencies {
                implementation("org.jetbrains.kotlin:kotlin-stdlib-common")

                // pbandk for protobuf runtime support
                implementation("pro.streem.pbandk:pbandk-runtime:0.16.0")
            }
        }

        commonTest.dependencies {
            implementation("org.jetbrains.kotlin:kotlin-test")
        }

        androidMain.dependencies {
            implementation("org.jetbrains.kotlin:kotlin-stdlib-jdk8")
        }
    }

    targets.withType<org.jetbrains.kotlin.gradle.plugin.mpp.KotlinNativeTarget> {
        compilations["main"].kotlinOptions.freeCompilerArgs += listOf(
            "-Xexport-kdoc",  // Let's add generation of comments which will be available in iOS
            "-Xexpect-actual-classes", // Let's suppress warnings about expect/actual functionality
        )
    }
}

android {
    namespace = "com.adguard.flm"
    compileSdk = 34

    ndkVersion = "28.1.13356709"

    defaultConfig {
        minSdk = 24

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
        targetSdk = 34
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
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
    androidTestImplementation("androidx.test.ext:junit:1.1.5")
    androidTestImplementation("androidx.test:core:1.5.0")
    androidTestImplementation("androidx.test.espresso:espresso-core:3.5.1")
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



publishing {
    repositories {
        mavenLocal()
    }
    publications {
        named<MavenPublication>("kotlinMultiplatform") {
            artifactId = "filter-list-manager"
            groupId = rootProject.group.toString()
            version = rootProject.version.toString()
        }
    }
}

afterEvaluate {
    publishing {
        publications {
            // Configure platform-specific publications
            withType<MavenPublication> {
                val targetName = name
                artifactId = when {
                    targetName == "kotlinMultiplatform" -> "filter-list-manager"
                    targetName.contains("android") -> "filter-list-manager-android"
                    else -> "filter-list-manager-$targetName"
                }
            }
        }
    }
}

file("publishing.gradle.kts").takeIf { it.exists() }?.let {
    apply(from = it)
}
