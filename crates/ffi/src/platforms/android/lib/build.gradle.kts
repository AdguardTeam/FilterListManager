import com.google.protobuf.gradle.id
import com.google.protobuf.gradle.proto

plugins {
    alias(libs.plugins.android.library)
    alias(libs.plugins.jetbrains.kotlin.android)
    alias(libs.plugins.google.protobuf)
    id("maven-publish")
}

version = "2.0-SNAPSHOT"

base {
    archivesName = "adguard-flm"
}

android {
    namespace = "com.adguard.flm"
    compileSdk = 34

    defaultConfig {
        minSdk = 24

        testInstrumentationRunner = "androidx.test.runner.AndroidJUnitRunner"
    }

    buildTypes {
        release {
            isMinifyEnabled = false
        }
    }
    externalNativeBuild {
        cmake {
            path("src/main/cpp/CMakeLists.txt")
        }
    }
    packaging {
        jniLibs {
            keepDebugSymbols.add("**/*.so")
        }
    }
    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_1_8
        targetCompatibility = JavaVersion.VERSION_1_8
    }
    kotlinOptions {
        jvmTarget = "1.8"
    }
    sourceSets["main"].proto {
        this.srcDirs("../../../protobuf")
    }
    ndkVersion = "28.1.13356709"
}

protobuf {
    protoc {
        artifact = "com.google.protobuf:protoc:${libs.versions.protobuf.get()}"
    }
    generateProtoTasks {
        all().forEach { task ->
            task.builtins {
                id("java") {
                    option("lite")
                } // For Java output
                id("kotlin") {
                    option("lite")
                } // Optional, if using KotlinProtobuf support
            }
        }
    }
}

dependencies {

    implementation(libs.protobuf.javalite)
    implementation(libs.protobuf.kotlin.lite)
    testImplementation(libs.junit)
    androidTestImplementation(libs.androidx.junit)
    androidTestImplementation(libs.androidx.espresso.core)
}

publishing {
    repositories {
        mavenLocal()
    }
    publications {
        create<MavenPublication>("release") {
            val buildDir by layout.buildDirectory
            groupId = "com.adguard.flm"
            artifactId = base.archivesName.get()
            version = "${project.version}"
            artifact(File("${buildDir}/outputs/aar/${artifactId}-release.aar")) {
                extension = "aar"
            }
            artifact(File("${buildDir}/libs/${artifactId}-${version}-sources.jar")) {
                classifier = "sources"
            }
        }
    }
}

afterEvaluate {
    tasks.matching { it.name.startsWith("publishReleasePublication") }
        .configureEach {
            dependsOn(tasks["releaseSourcesJar"])
        }
}

file("publishing.gradle.kts").takeIf { it.exists() }?.let {
    apply(from = it)
}
