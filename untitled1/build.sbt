import sbt.Compile

ThisBuild / version := "0.1.0-SNAPSHOT"

ThisBuild / scalaVersion := "3.3.7"

// project/plugins.sbt
lazy val root = (project in file("."))
  .settings(
    name := "untitled1",
    libraryDependencies ++= Seq(
      "org.scala-lang" %% "toolkit" % "0.7.0"
    )
  )

