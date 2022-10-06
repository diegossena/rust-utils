rustc src/main.rs
IF %ERRORLEVEL% == 0 (
  cd bin
  main
  cd..
)