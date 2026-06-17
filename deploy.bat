@echo off
REM Sistema de Gestión Escolar - Script de Despliegue
REM Para Windows
REM Uso: deploy.bat [compile|start|stop|status]

setlocal enabledelayedexpansion

REM Configuración por defecto
set BINARY_NAME=escuela_api.exe
set DATABASE_PATH=escuela.db
set STORAGE_PATH=storage
set BIND_ADDRESS=0.0.0.0:3000
set RELEASE_DIR=target\release

REM Función para mostrar mensajes
:print_info
echo [INFO] %~1
goto :eof

:print_success
echo [SUCCESS] %~1
goto :eof

:print_warning
echo [WARNING] %~1
goto :eof

:print_error
echo [ERROR] %~1
goto :eof

REM Función para compilar el proyecto
:compile_project
call :print_info "Iniciando compilación del proyecto..."

REM Verificar si Rust está instalado
where cargo >nul 2>nul
if %errorlevel% neq 0 (
    call :print_error "Cargo no encontrado. Por favor instala Rust primero."
    call :print_info "Visita: https://rustup.rs/ para instrucciones de instalación"
    exit /b 1
)

REM Compilar en modo release
call :print_info "Ejecutando: cargo build --release"
cargo build --release

if %errorlevel% equ 0 (
    call :print_success "Compilación completada exitosamente"
    call :print_info "Binario generado: %RELEASE_DIR%\%BINARY_NAME%"
) else (
    call :print_error "La compilación falló"
    exit /b 1
)
goto :eof

REM Función para verificar el binario
:verify_binary
call :print_info "Verificando binario compilado..."

if not exist "%RELEASE_DIR%\%BINARY_NAME%" (
    call :print_warning "Binario no encontrado en: %RELEASE_DIR%\%BINARY_NAME%"
    call :print_info "Iniciando compilación..."
    call :compile_project
) else (
    call :print_success "Binario encontrado: %RELEASE_DIR%\%BINARY_NAME%"
    
    REM Mostrar información del binario
    for %%A in ("%RELEASE_DIR%\%BINARY_NAME%") do (
        set SIZE=%%~zA
        set /a SIZE_MB=!SIZE! / 1048576
        call :print_info "Tamaño del binario: !SIZE! MB"
    )
)
goto :eof

REM Función para configurar variables de entorno
:setup_environment
call :print_info "Configurando variables de entorno..."

set DATABASE_PATH=%DATABASE_PATH%
set STORAGE_PATH=%STORAGE_PATH%
set BIND_ADDRESS=%BIND_ADDRESS%

call :print_success "Variables de entorno configuradas:"
call :print_info "  DATABASE_PATH=%DATABASE_PATH%"
call :print_info "  STORAGE_PATH=%STORAGE_PATH%"
call :print_info "  BIND_ADDRESS=%BIND_ADDRESS%"
goto :eof

REM Función para iniciar el servidor
:start_server
call :print_info "Iniciando servidor del Sistema de Gestión Escolar..."

REM Verificar si ya está corriendo
tasklist /FI "IMAGENAME eq %BINARY_NAME%" 2>NUL | find /I /N "%BINARY_NAME%">NUL
if %errorlevel% equ 0 (
    call :print_warning "El servidor ya está corriendo"
    for /f "tokens=2" %%A in ('tasklist /FI "IMAGENAME eq %BINARY_NAME%" ^| find /I "%BINARY_NAME%"') do (
        call :print_info "PID: %%A"
    )
    goto :eof
)

REM Crear directorio de almacenamiento si no existe
if not exist "%STORAGE_PATH%" (
    mkdir "%STORAGE_PATH%"
)

REM Iniciar el servidor en background
cd /d "%RELEASE_DIR%"
start /B "" %BINARY_NAME% > ..\server.log 2>&1

REM Esperar un momento para verificar que inició correctamente
timeout /t 2 /nobreak >nul

tasklist /FI "IMAGENAME eq %BINARY_NAME%" 2>NUL | find /I /N "%BINARY_NAME%">NUL
if %errorlevel% equ 0 (
    call :print_success "Servidor iniciado exitosamente"
    for /f "tokens=2" %%A in ('tasklist /FI "IMAGENAME eq %BINARY_NAME%" ^| find /I "%BINARY_NAME%"') do (
        call :print_info "PID: %%A"
    )
    call :print_info "Logs: server.log"
    call :print_info "URL: http://%BIND_ADDRESS%"
) else (
    call :print_error "El servidor falló al iniciar"
    call :print_info "Revisar logs en: server.log"
    exit /b 1
)
goto :eof

REM Función para detener el servidor
:stop_server
call :print_info "Deteniendo servidor..."

tasklist /FI "IMAGENAME eq %BINARY_NAME%" 2>NUL | find /I /N "%BINARY_NAME%">NUL
if %errorlevel% equ 0 (
    taskkill /F /IM %BINARY_NAME% >nul 2>&1
    call :print_success "Servidor detenido"
) else (
    call :print_warning "El servidor no está corriendo"
)
goto :eof

REM Función para verificar estado del servidor
:check_status
call :print_info "Verificando estado del servidor..."

tasklist /FI "IMAGENAME eq %BINARY_NAME%" 2>NUL | find /I /N "%BINARY_NAME%">NUL
if %errorlevel% equ 0 (
    call :print_success "Servidor está corriendo"
    for /f "tokens=2" %%A in ('tasklist /FI "IMAGENAME eq %BINARY_NAME%" ^| find /I "%BINARY_NAME%"') do (
        call :print_info "PID: %%A"
    )
    
    REM Verificar si el puerto está escuchando
    for /f "tokens=2 delims=:" %%P in ("%BIND_ADDRESS%") do (
        set PORT=%%P
    )
    netstat -an | findstr ":!PORT! " >nul
    if %errorlevel% equ 0 (
        call :print_success "Puerto !PORT! está escuchando"
    ) else (
        call :print_warning "Puerto !PORT! no está escuchando"
    )
) else (
    call :print_warning "Servidor no está corriendo"
)
goto :eof

REM Función principal
:main
if "%1"=="" (
    set COMMAND=start
) else (
    set COMMAND=%1
)

if "%COMMAND%"=="compile" (
    call :print_info "Modo: Compilación"
    call :compile_project
) else if "%COMMAND%"=="start" (
    call :print_info "Modo: Iniciar servidor"
    call :verify_binary
    call :setup_environment
    call :start_server
) else if "%COMMAND%"=="stop" (
    call :print_info "Modo: Detener servidor"
    call :stop_server
) else if "%COMMAND%"=="status" (
    call :print_info "Modo: Verificar estado"
    call :check_status
) else if "%COMMAND%"=="restart" (
    call :print_info "Modo: Reiniciar servidor"
    call :stop_server
    timeout /t 1 /nobreak >nul
    call :verify_binary
    call :setup_environment
    call :start_server
) else if "%COMMAND%"=="deploy" (
    call :print_info "Modo: Despliegue completo"
    call :compile_project
    call :setup_environment
    call :start_server
) else (
    echo Uso: %0 [compile^|start^|stop^|status^|restart^|deploy]
    echo.
    echo Comandos:
    echo   compile  - Compila el proyecto en modo release
    echo   start    - Inicia el servidor
    echo   stop     - Detiene el servidor
    echo   status   - Verifica el estado del servidor
    echo   restart  - Reinicia el servidor
    echo   deploy   - Compila e inicia el servidor (despliegue completo)
    exit /b 1
)

goto :eof

REM Ejecutar función principal
call :main %*
