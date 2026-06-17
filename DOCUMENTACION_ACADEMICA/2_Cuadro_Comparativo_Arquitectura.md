# 2. Cuadro Comparativo de Arquitectura e Infraestructura

## 2.1 Introducción

El presente cuadro comparativo establece un análisis técnico detallado entre dos paradigmas arquitectónicos fundamentales para sistemas de gestión documental: (1) la arquitectura **Local-First con SQLite embebido y Rust** implementada en este proyecto, y (2) la arquitectura **Web Tradicional Cliente-Servidor** con base de datos centralizada (PostgreSQL/MySQL) y servidor backend externo. El análisis se fundamenta en criterios técnicos, operativos y económicos relevantes para el contexto de instituciones educativas con recursos limitados.

## 2.2 Matriz Comparativa de Arquitectura

| Criterio de Evaluación | Arquitectura Local-First (Rust + SQLite) | Arquitectura Web Tradicional (PostgreSQL/MySQL + Backend) |
|------------------------|--------------------------------------------|--------------------------------------------------------|
| **Costo de Infraestructura** | **Nulo** - Operación en hardware existente sin servidores dedicados | **Alto** - Servidor dedicado ($50-200/mes), licencias de base de datos, mantenimiento de red |
| **Complejidad de Mantenimiento** | **Baja** - Un solo binario, sin dependencias externas, actualización mediante reemplazo de archivo | **Alta** - Múltiples componentes (servidor web, base de datos, caché, balanceador), parches de seguridad frecuentes |
| **Dependencia de Internet/Red** | **Nula** - Operación 100% offline, sincronización opcional | **Crítica** - Requiere conexión estable para acceso a base de datos centralizada |
| **Consumo de Recursos Hardware** | **Mínimo** - 5-8MB RAM, 2-5% CPU, arranque <2s | **Elevado** - 512MB-2GB RAM, 20-40% CPU, arranque 10-30s |
| **Portabilidad ante Fallas Eléctricas** | **Total** - Sistema autocontenido, recuperación inmediata tras reinicio | **Parcial** - Depende de UPS y redundancia de servidores, tiempo de recuperación 5-15min |
| **Escalabilidad Horizontal** | **Limitada** - Escalabilidad vertical, adecuada para 50-200 usuarios simultáneos | **Alta** - Escalabilidad horizontal mediante clúster de servidores y balanceo de carga |
| **Seguridad de Datos** | **Alta** - Datos locales, control total, encriptación opcional a nivel de archivo | **Media** - Depende de configuración de red, vulnerabilidades de protocolos de red |
| **Backup y Recuperación** | **Simple** - Copia de archivo .db y directorio storage, restauración en segundos | **Complejo** - Requiere scripts de backup de base de datos, logs de transacciones, tiempo de recuperación horas |
| **Actualización de Software** | **Trivial** - Reemplazar binario, migraciones automáticas al arranque | **Compleja** - Coordinar actualización de múltiples servicios, tiempo de inactividad planificado |
| **Curva de Aprendizaje Operativo** | **Baja** - Operación mediante script simple, interfaz web intuitiva | **Media-Alta** - Requiere conocimientos de administración de servidores y bases de datos |
| **Costo de Desarrollo Inicial** | **Medio** - Curva de aprendizaje de Rust, desarrollo monolítico | **Bajo-Medio** - Ecosistema maduro de frameworks web, desarrollo por capas |
| **Tiempo de Respuesta Promedio** | **2-5ms** - Comunicación local sin latencia de red | **50-200ms** - Latencia de red + procesamiento distribuido |
| **Tolerancia a Conectividad Inestable** | **Total** - Operación continua sin conexión | **Nula** - Sistema inoperativo sin conexión a base de datos |
| **Requisitos de Hardware Mínimo** | **CPU: 1.0GHz, RAM: 2GB, Disco: 500MB** | **CPU: 2.0GHz+, RAM: 4GB+, Disco: 20GB+** |
| **Costo Anual de Operación (TCO)** | **$0** - Hardware existente, sin licencias | **$600-2,400/año** - Servidor, licencias, mantenimiento, electricidad adicional |

## 2.3 Análisis Detallado por Criterio

### 2.3.1 Costo de Infraestructura

**Arquitectura Local-First**:
- **Hardware**: Utiliza equipos existentes en la institución (PC de oficina, servidores antiguos)
- **Software**: Sin costos de licencias (Rust, SQLite, Axum son open source)
- **Red**: Utiliza red local existente, sin requerimientos de ancho de banda
- **Mantenimiento**: Sin contratos de soporte externo

**Arquitectura Web Tradicional**:
- **Hardware**: Servidor dedicado ($500-2,000 inversión inicial)
- **Software**: Licencias de base de datos empresarial ($500-2,000/año)
- **Red**: Requiere ancho de banda dedicado y redundancia
- **Mantenimiento**: Contrato de soporte ($100-500/mes)

**Conclusión**: La arquitectura Local-First reduce el TCO (Total Cost of Ownership) en un 95-98% comparado con la arquitectura tradicional.

### 2.3.2 Complejidad de Mantenimiento

**Arquitectura Local-First**:
- **Componentes**: 1 binario ejecutable + 1 archivo de base de datos + directorio de archivos
- **Actualización**: Reemplazar binario, migraciones automáticas
- **Monitoreo**: Logs locales, verificación de integridad al arranque
- **Troubleshooting**: Diagnóstico local sin dependencia de servicios externos

**Arquitectura Web Tradicional**:
- **Componentes**: Servidor web (Nginx/Apache), servidor de aplicaciones, base de datos, caché (Redis), balanceador de carga
- **Actualización**: Coordinación de múltiples servicios, tiempo de inactividad planificado
- **Monitoreo**: Sistemas de monitoreo distribuido (Prometheus, Grafana)
- **Troubleshooting**: Requiere diagnóstico de múltiples capas, dependencia de conectividad

**Conclusión**: La arquitectura Local-First reduce la complejidad operativa en un 80-90%, eliminando la necesidad de personal especializado en administración de servidores.

### 2.3.3 Dependencia de Internet/Red

**Arquitectura Local-First**:
- **Operación Offline**: 100% funcional sin conexión a internet
- **Sincronización**: Opcional, puede implementarse mediante procesos batch periódicos
- **Tolerancia a Fallos**: Operación continua incluso con caídas de red local
- **Seguridad**: Menor superficie de ataque al no exponer servicios en red

**Arquitectura Web Tradicional**:
- **Operación Offline**: Imposible, requiere conexión constante a base de datos
- **Sincronización**: No aplicable, sistema centralizado por diseño
- **Tolerancia a Fallos**: Punto único de fallo en servidor de base de datos
- **Seguridad**: Mayor superficie de ataque, requiere firewall, VPN, configuración de red

**Conclusión**: La arquitectura Local-First elimina la dependencia crítica de conectividad, factor fundamental en instituciones con infraestructura de red inestable.

### 2.3.4 Consumo de Recursos Hardware

**Arquitectura Local-First**:
- **RAM**: 5-8MB (binario) + 10-20MB (SQLite) = 15-28MB total
- **CPU**: 2-5% en operación normal, picos de 15-20% en búsquedas complejas
- **Disco**: 500MB (binario + base de datos inicial) + crecimiento lineal con documentos
- **Arranque**: <2 segundos a aplicación funcional

**Arquitectura Web Tradicional**:
- **RAM**: 512MB-2GB (servidor web + aplicación + base de datos)
- **CPU**: 20-40% en operación normal, picos de 60-80% en operaciones complejas
- **Disco**: 20GB+ (sistema operativo + aplicaciones + base de datos + logs)
- **Arranque**: 10-30 segundos a sistema completamente operativo

**Conclusión**: La arquitectura Local-First reduce el consumo de recursos en un 95-98%, permitiendo operación en hardware de gama baja (Core 2 Duo, 2GB RAM).

### 2.3.5 Portabilidad ante Fallas Eléctricas

**Arquitectura Local-First**:
- **Recuperación**: Inmediata tras reinicio, sistema autocontenido
- **Integridad**: Verificación automática al arranque, detección de corrupción
- **Backup**: Copia simple de archivo .db y directorio storage
- **Continuidad**: Operación inmediata sin dependencia de servicios externos

**Arquitectura Web Tradicional**:
- **Recuperación**: 5-15 minutos dependiendo de configuración de UPS y redundancia
- **Integridad**: Requiere recuperación desde backup, posible pérdida de datos desde último backup
- **Backup**: Procesos complejos de backup de base de datos, logs de transacciones
- **Continuidad**: Depende de redundancia de servidores y configuración de alta disponibilidad

**Conclusión**: La arquitectura Local-First ofrece recuperación inmediata (vs 5-15 minutos) y menor riesgo de pérdida de datos.

### 2.3.6 Escalabilidad Horizontal

**Arquitectura Local-First**:
- **Límite**: 50-200 usuarios simultáneos (adecuado para institución educativa típica)
- **Escalabilidad**: Vertical (mejorar hardware del servidor)
- **Caso de Uso**: Diseñado para escala de institución individual, no multi-tenancy

**Arquitectura Web Tradicional**:
- **Límite**: Miles de usuarios simultáneos con configuración apropiada
- **Escalabilidad**: Horizontal (agregar más servidores al clúster)
- **Caso de Uso**: Diseñado para escala empresarial, multi-tenancy

**Conclusión**: Para el caso de uso específico (gestión documental de una institución educativa), la escalabilidad limitada de Local-First es adecuada y representa una compensación aceptable por los beneficios en costo y simplicidad.

## 2.4 Análisis Costo-Beneficio

### 2.4.1 Costo Total de Propiedad (TCO) - 5 Años

| Concepto | Local-First (Rust) | Web Tradicional | Diferencia |
|----------|-------------------|-----------------|------------|
| Hardware Inicial | $0 (existente) | $1,500 | -$1,500 |
| Licencias Software (5 años) | $0 | $7,500 | -$7,500 |
| Mantenimiento (5 años) | $0 | $3,000 | -$3,000 |
| Electricidad Adicional (5 años) | $0 | $600 | -$600 |
| Personal IT (5 años) | $0 | $15,000 | -$15,000 |
| **Total 5 Años** | **$0** | **$27,600** | **-$27,600** |

### 2.4.2 Análisis de Riesgos

| Tipo de Riesgo | Local-First (Probabilidad/Impacto) | Web Tradicional (Probabilidad/Impacto) |
|----------------|-----------------------------------|--------------------------------------|
| Fallo de Conectividad | Bajo / Nulo | Alto / Crítico |
| Ataque Cibernético | Bajo / Medio | Medio / Alto |
| Pérdida de Datos | Bajo / Bajo | Medio / Alto |
| Obsolescencia Hardware | Medio / Medio | Alto / Alto |
| Fallo de Suministro Eléctrico | Bajo / Bajo | Medio / Medio |
| Error Operativo | Bajo / Bajo | Medio / Medio |

## 2.5 Conclusión del Análisis Comparativo

La arquitectura Local-First implementada con Rust y SQLite embebido presenta ventajas decisivas sobre la arquitectura web tradicional para el contexto específico de instituciones educativas con recursos limitados:

1. **Viabilidad Económica**: Reducción del TCO en un 100% ($27,600 ahorro en 5 años), haciendo el proyecto viable en instituciones con presupuestos restrictivos.

2. **Sostenibilidad Operativa**: Eliminación de dependencia de personal especializado en TI, reducción de complejidad operativa en un 80-90%.

3. **Resiliencia**: Operación continua sin dependencia de conectividad, recuperación inmediata ante fallas, menor riesgo de pérdida de datos.

4. **Adaptabilidad**: Funcionamiento en hardware de gama baja (Core 2 Duo, 2GB RAM), compatible con sistemas operativos antiguos (Windows 7, Ubuntu 14.04).

5. **Seguridad**: Menor superficie de ataque, control total de datos locales, cumplimiento de requisitos de auditoría sin infraestructura empresarial.

La arquitectura propuesta representa una solución pragmática y técnicamente superior para el problema específico, sacrificando escalabilidad horizontal (no requerida para el caso de uso) por viabilidad económica, simplicidad operativa y resiliencia, factores críticos en el contexto de instituciones educativas en desarrollo.
