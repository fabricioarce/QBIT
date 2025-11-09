# Comando !balance

## Descripción

El comando `!balance` muestra tu balance actual de **monedas** en el servidor. Las monedas se ganan resolviendo problemas de Codeforces con el comando `!solved` y pueden usarse en futuras funcionalidades del bot.

## Uso

```
!balance
```

### Parámetros

Este comando no requiere parámetros adicionales.

### Ejemplos

```
!balance
```

## Funcionalidad

Cuando ejecutas el comando, el bot:

1. **Consulta la base de datos**: Busca tu perfil en el servidor actual
2. **Obtiene tu balance**: Recupera la cantidad actual de monedas
3. **Muestra la información**: Presenta tu balance de forma clara y amigable

## Respuestas del Bot

### Balance Disponible 💰
```
💰 Balance de monedas

👤 Usuario: @tu_usuario
🪙 Monedas: `15`

💡 Resuelve problemas de Codeforces con `!solved` para ganar más monedas!
```

### Perfil No Encontrado ❌
```
❌ No tienes un perfil en este servidor. Usa `!account tu_handle` para crear uno y comenzar a ganar monedas.
```

### Error de Base de Datos ❌
```
❌ Error al acceder a la base de datos. Intenta de nuevo más tarde.
```

## Sistema de Monedas 💰

### ¿Cómo Ganar Monedas?

Actualmente puedes ganar monedas de las siguientes maneras:

- **Resolver problemas**: `+1 moneda` por cada problema de Codeforces verificado con `!solved`
- **Futuras actividades**: Participación en eventos, desafíos especiales, etc.

### ¿Para Qué Sirven las Monedas?

Las monedas están diseñadas para ser utilizadas en:

- **Funcionalidades futuras**: Tienda del servidor, recompensas especiales
- **Sistema de logros**: Desbloquear títulos o reconocimientos
- **Eventos especiales**: Participar en competencias exclusivas
- **Personalización**: Modificar aspectos de tu perfil

### Reglas del Sistema

- ✅ **Acumulativas**: Las monedas se acumulan sin límite
- ✅ **Permanentes**: No se pierden con el tiempo
- ✅ **Por servidor**: Cada servidor tiene su propio balance
- ❌ **No transferibles**: No se pueden enviar entre usuarios (por ahora)
- ❌ **Una por problema**: Solo recibes monedas la primera vez que resuelves cada problema

## Ejemplos de Uso

### Verificar Balance Inicial
```
Usuario: !balance
Bot: ❌ No tienes un perfil en este servidor. Usa `!account tu_handle` para crear uno y comenzar a ganar monedas.
```

### Después de Crear Cuenta
```
Usuario: !account mi_handle
Bot: ✅ Cuenta de Codeforces vinculada exitosamente! [...]

Usuario: !balance
Bot: 💰 Balance de monedas - 👤 Usuario: @usuario - 🪙 Monedas: `0`
```

### Después de Resolver Problemas
```
Usuario: !solved 467B
Bot: 🎉 Problema resuelto verificado! [...] 💰 +1 moneda ganada!

Usuario: !balance
Bot: 💰 Balance de monedas - 👤 Usuario: @usuario - 🪙 Monedas: `1`
```

### Usuario Experimentado
```
Usuario: !balance
Bot: 💰 Balance de monedas - 👤 Usuario: @usuario - 🪙 Monedas: `47`
```

## Integración con Otros Comandos

### Comandos Relacionados

- **`!account`**: Requerido para tener un perfil y comenzar a acumular monedas
- **`!solved`**: Forma principal de ganar monedas (+1 por problema resuelto)
- **`!problem`**: Obtener problemas nuevos para resolver y ganar monedas

### Flujo Típico de Usuario

1. **Crear perfil**: `!account mi_handle_cf`
2. **Verificar balance inicial**: `!balance` → 0 monedas
3. **Resolver problemas**: `!solved 467B` → +1 moneda
4. **Verificar progreso**: `!balance` → 1 moneda
5. **Repetir**: Seguir resolviendo más problemas

## Notas Importantes

- 📊 **Balance por servidor**: Cada servidor Discord tiene su propio sistema de monedas independiente
- 🔄 **Actualización en tiempo real**: El balance se actualiza inmediatamente cuando ganas monedas
- 🎯 **Motivación**: Las monedas incentivan la práctica constante de competitive programming
- 🚀 **Futuro**: Se planean más formas de ganar y gastar monedas

## Estadísticas y Progreso

El comando `!balance` es útil para:

- **Seguimiento personal**: Ver tu progreso a lo largo del tiempo
- **Motivación**: Establecer metas de monedas a alcanzar
- **Comparación**: Conocer tu progreso antes de competir con otros
- **Planificación**: Saber cuántas monedas tienes disponibles para futuras funcionalidades

## Privacidad

- 🔒 **Solo tu balance**: Solo puedes ver tu propio balance de monedas
- 👥 **Sin rankings públicos**: El comando no revela el balance de otros usuarios
- 📝 **Datos locales**: La información se almacena solo en la base de datos del servidor

## Solución de Problemas

### El comando no responde
1. Verifica que el bot tenga permisos para responder en el canal
2. Asegúrate de estar en un servidor donde el bot esté activo
3. Intenta de nuevo en unos minutos

### Muestra 0 monedas aunque he resuelto problemas
1. Verifica que hayas usado `!solved` correctamente para marcar los problemas
2. Confirma que los problemas fueron verificados exitosamente
3. Recuerda que solo cuentan los problemas con veredicto "OK" en Codeforces

### Error de base de datos
1. Es un problema temporal del servidor
2. Intenta de nuevo más tarde
3. Contacta a los administradores si persiste

---

*El comando `!balance` te ayuda a hacer seguimiento de tu progreso en competitive programming y te prepara para futuras funcionalidades del bot.*
