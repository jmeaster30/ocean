plugins {
  id("java")
  id("org.jetbrains.kotlin.jvm") version "1.9.0"
  id("org.jetbrains.intellij") version "1.15.0"
  id("org.jetbrains.grammarkit") version "2022.3.2"
}

group = "com.syrency"
version = "1.0-SNAPSHOT"

repositories {
  mavenCentral()
}

sourceSets {
  main {
    java {
      srcDirs("src/main/gen")
    }
  }
}

// Configure Gradle IntelliJ Plugin
// Read more: https://plugins.jetbrains.com/docs/intellij/tools-gradle-intellij-plugin.html
intellij {
  version.set("2022.2.5")
  type.set("IC") // Target IDE Platform

  plugins.set(listOf(/* Plugin Dependencies */))
}

tasks {
  generateLexer {
    sourceFile.set(file("src/main/java/com/syrency/ocean/language/Hydro.flex"))
    targetDir.set("src/main/gen/com/syrency/ocean/language/")
    targetClass.set("HydroLexer")
    purgeOldFiles.set(true)
  }
  generateParser {
    sourceFile.set(file("src/main/java/com/syrency/ocean/language/Hydro.bnf"))
    targetRoot.set("src/main/gen")
    pathToParser.set("com/syrency/ocean/language/parser/HydroParser.java")
    pathToPsiRoot.set("com/syrency/ocean/language/psi")
    purgeOldFiles.set(true)
  }
  // Set the JVM compatibility versions
  withType<JavaCompile> {
    dependsOn(generateLexer, generateParser)
    sourceCompatibility = "17"
    targetCompatibility = "17"
  }
  withType<org.jetbrains.kotlin.gradle.tasks.KotlinCompile> {
    kotlinOptions.jvmTarget = "17"
  }

  patchPluginXml {
    sinceBuild.set("222")
    untilBuild.set("")
  }

  signPlugin {
    certificateChain.set(System.getenv("CERTIFICATE_CHAIN"))
    privateKey.set(System.getenv("PRIVATE_KEY"))
    password.set(System.getenv("PRIVATE_KEY_PASSWORD"))
  }

  publishPlugin {
    token.set(System.getenv("PUBLISH_TOKEN"))
  }
}
