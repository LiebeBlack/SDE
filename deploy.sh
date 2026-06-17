#!/bin/bash

# Sistema de Gestión Escolar - Script de Despliegue
# Para Linux/macOS
# Uso: ./deploy.sh [compile|start|stop|status]

set -e

# Colores para output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuración por defecto
BINARY_NAME="escuela_api"
DATABASE_PATH="escuela.db"
STORAGE_PATH="storage"
BIND_ADDRESS="0.0.0.0:3000"
RELEASE_DIR="target/release"

# Función para mostrar mensajes
print_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Función para compilar el proyecto
compile_project() {
    print_info "Iniciando compilación del proyecto..."
    
    # Verificar si Rust está instalado
    if ! command -v cargo &> /dev/null; then
        print_error "Cargo no encontrado. Por favor instala Rust primero."
        print_info "Visita: https://rustup.rs/ para instrucciones de instalación"
        exit 1
    fi
    
    # Compilar en modo release
    print_info "Ejecutando: cargo build --release"
    cargo build --release
    
    if [ $? -eq 0 ]; then
        print_success "Compilación completada exitosamente"
        print_info "Binario generado: $RELEASE_DIR/$BINARY_NAME"
    else
        print_error "La compilación falló"
        exit 1
    fi
}

# Función para verificar el binario
verify_binary() {
    print_info "Verificando binario compilado..."
    
    if [ ! -f "$RELEASE_DIR/$BINARY_NAME" ]; then
        print_warning "Binario no encontrado en: $RELEASE_DIR/$BINARY_NAME"
        print_info "Iniciando compilación..."
        compile_project
    else
        print_success "Binario encontrado: $RELEASE_DIR/$BINARY_NAME"
        
        # Mostrar información del binario
        if command -v ls &> /dev/null; then
            SIZE=$(ls -lh "$RELEASE_DIR/$BINARY_NAME" | awk '{print $5}')
            print_info "Tamaño del binario: $SIZE"
        fi
    fi
}

# Función para configurar variables de entorno
setup_environment() {
    print_info "Configurando variables de entorno..."
    
    export DATABASE_PATH="${DATABASE_PATH}"
    export STORAGE_PATH="${STORAGE_PATH}"
    export BIND_ADDRESS="${BIND_ADDRESS}"
    
    print_success "Variables de entorno configuradas:"
    print_info "  DATABASE_PATH=$DATABASE_PATH"
    print_info "  STORAGE_PATH=$STORAGE_PATH"
    print_info "  BIND_ADDRESS=$BIND_ADDRESS"
}

# Función para iniciar el servidor
start_server() {
    print_info "Iniciando servidor del Sistema de Gestión Escolar..."
    
    # Verificar si ya está corriendo
    if pgrep -f "$BINARY_NAME" > /dev/null; then
        print_warning "El servidor ya está corriendo"
        print_info "PID: $(pgrep -f $BINARY_NAME)"
        return 0
    fi
    
    # Crear directorio de almacenamiento si no existe
    mkdir -p "$STORAGE_PATH"
    
    # Iniciar el servidor en background
    cd "$RELEASE_DIR"
    nohup ./$BINARY_NAME > ../server.log 2>&1 &
    SERVER_PID=$!
    
    # Esperar un momento para verificar que inició correctamente
    sleep 2
    
    if ps -p $SERVER_PID > /dev/null; then
        print_success "Servidor iniciado exitosamente"
        print_info "PID: $SERVER_PID"
        print_info "Logs: server.log"
        print_info "URL: http://$BIND_ADDRESS"
    else
        print_error "El servidor falló al iniciar"
        print_info "Revisar logs en: server.log"
        exit 1
    fi
}

# Función para detener el servidor
stop_server() {
    print_info "Deteniendo servidor..."
    
    if pgrep -f "$BINARY_NAME" > /dev/null; then
        pkill -f "$BINARY_NAME"
        print_success "Servidor detenido"
    else
        print_warning "El servidor no está corriendo"
    fi
}

# Función para verificar estado del servidor
check_status() {
    print_info "Verificando estado del servidor..."
    
    if pgrep -f "$BINARY_NAME" > /dev/null; then
        PID=$(pgrep -f $BINARY_NAME)
        print_success "Servidor está corriendo"
        print_info "PID: $PID"
        
        # Verificar si el puerto está escuchando
        PORT=$(echo $BIND_ADDRESS | cut -d':' -f2)
        if command -v netstat &> /dev/null; then
            if netstat -tuln | grep -q ":$PORT "; then
                print_success "Puerto $PORT está escuchando"
            else
                print_warning "Puerto $PORT no está escuchando"
            fi
        fi
    else
        print_warning "Servidor no está corriendo"
    fi
}

# Función principal
main() {
    case "${1:-start}" in
        compile)
            print_info "Modo: Compilación"
            compile_project
            ;;
        start)
            print_info "Modo: Iniciar servidor"
            verify_binary
            setup_environment
            start_server
            ;;
        stop)
            print_info "Modo: Detener servidor"
            stop_server
            ;;
        status)
            print_info "Modo: Verificar estado"
            check_status
            ;;
        restart)
            print_info "Modo: Reiniciar servidor"
            stop_server
            sleep 1
            verify_binary
            setup_environment
            start_server
            ;;
        deploy)
            print_info "Modo: Despliegue completo"
            compile_project
            setup_environment
            start_server
            ;;
        *)
            echo "Uso: $0 {compile|start|stop|status|restart|deploy}"
            echo ""
            echo "Comandos:"
            echo "  compile  - Compila el proyecto en modo release"
            echo "  start    - Inicia el servidor"
            echo "  stop     - Detiene el servidor"
            echo "  status   - Verifica el estado del servidor"
            echo "  restart  - Reinicia el servidor"
            echo "  deploy   - Compila e inicia el servidor (despliegue completo)"
            exit 1
            ;;
    esac
}

# Ejecutar función principal
main "$@"
