name := "sbt-test"

version := "0.1.0"

scalaVersion := "2.13.12"

// 添加一些简单的依赖
libraryDependencies ++= Seq(
  "org.scala-lang" % "scala-library" % scalaVersion.value,
  "com.typesafe" % "config" % "1.4.3"
)