# Comando !solved

## Descripción

El comando `!solved` permite verificar si has resuelto exitosamente un problema específico de **Codeforces** y lo marca como completado en tu perfil del servidor. El bot consulta automáticamente tus submissions en Codeforces para confirmar que el problema fue resuelto correctamente.

## Uso

```
!solved <contest_id><index>
```

### Parámetros

- `<contest_id>`: El ID numérico del contest de Codeforces
- `<index>`: La letra o índice del problema (A, B, C, D, etc.)

**Formato:** Los parámetros deben escribirse juntos, sin espacios (ej: `467B`, `1200A`, `842D`)

### Ejemplos

```
!solved 467B
!solved 1200A
!solved 842D2
!solved 1500C
```

## Requisitos Previos

⚠️ **Importante:** Para usar este comando, primero debes tener tu cuenta de Codeforces vinculada:

```
!account tu_handle_de_codeforces
```

## Funcionalidad

Cuando ejecutas el comando, el bot:

1. **Valida el formato**: Verifica que el problema tenga el formato correcto (número + letra/índice)
2. **Verifica la vinculación**: Confirma que tengas una cuenta de Codeforces asociada
3. **Consulta la base de datos**: Verifica si el problema ya está marcado como resuelto
4. **Consulta la API**: Se conecta a `https://codeforces.com/api/user.status?handle=tu_handle`
5. **Busca submissions**: Examina todas tus submissions para encontrar una solución exitosa
6. **Verifica el veredicto**: Confirma que el veredicto sea "OK" (Accepted)
7. **Actualiza la base de datos**: Marca el problema como resuelto si se encuentra una solución exitosa

## Criterios de Verificación

Para que un problema se considere resuelto, debe cumplir:

- ✅ **Contest ID correcto**: La submission debe ser del contest especificado
- ✅ **Índice correcto**: La submission debe ser del problema específico (A, B, C, etc.)
- ✅ **Veredicto "OK"**: La submission debe haber sido aceptada completamente
- ✅ **Pertenencia del usuario**: La submission debe ser tuya

## Respuestas del Bot

### Éxito ✅
```
🎉 Problema resuelto verificado!

✅ El problema `467B` ha sido marcado como resuelto.
👤 Usuario: `tu_handle`
🏆 ¡Felicitaciones por la solución exitosa!
```

### Problema No Resuelto ❌
```
❌ Problema no resuelto

🔍 No se encontró una solución exitosa para el problema `467B`
👤 Usuario: `tu_handle`

💡 Posibles razones:
• El problema no ha sido resuelto aún
• La solución no pasó todos los test cases
• El problema no existe o el formato es incorrecto

¡Sigue intentando! 💪
```

### Errores Comunes

**Formato incorrecto:**
```
❌ Formato de problema inválido. Usa el formato: `467B` (contest_id + index)
```

**Parámetro vacío:**
```
❌ Por favor proporciona un ID de problema. Uso: `!solved 467B`
```

**Cuenta no vinculada:**
```
❌ No tienes una cuenta de Codeforces vinculada. Usa `!account tu_handle` primero.
```

**Ya marcado como resuelto:**
```
ℹ️ El problema `467B` ya está marcado como resuelto.
```

**Contest ID inválido:**
```
❌ Contest ID inválido. Debe ser un número.
```

**Error de API:**
```
❌ Error al conectar con la API de Codeforces. Verifica que tu handle sea correcto e intenta más tarde.
```

## Formatos de Problema Válidos

### ✅ Formatos Correctos
- `467B` - Contest 467, problema B
- `1200A` - Contest 1200, problema A  
- `842D2` - Contest 842, problema D2
- `1500C` - Contest 1500, problema C
- `100A` - Contest 100, problema A

### ❌ Formatos Incorrectos
- `467 B` - No debe tener espacios
- `B467` - El contest ID debe ir primero
- `467` - Falta el índice del problema
- `B` - Falta el contest ID

## Notas Importantes

- 🔄 **Verificación automática**: El bot busca automáticamente en todas tus submissions
- 📊 **Solo submissions aceptadas**: Solo cuenta si el veredicto es "OK"
- 🚫 **No duplicados**: Si ya está marcado como resuelto, no se procesa nuevamente
- 🌐 **Dependencia de API**: Requiere que la API de Codeforces esté disponible
- ⏱️ **Tiempo real**: La verificación se hace con datos actuales de tus submissions
- 🔐 **Perfil público**: Tu perfil de Codeforces debe ser público para la verificación

## Casos de Uso

### Escenario 1: Problema Recién Resuelto
```
Usuario: !solved 1200A
Bot: 🎉 Problema resuelto verificado! ✅ El problema `1200A` ha sido marcado como resuelto.
```

### Escenario 2: Problema Intentado Pero No Resuelto
```
Usuario: !solved 1500D
Bot: ❌ Problema no resuelto. No se encontró una solución exitosa para el problema `1500D`
```

### Escenario 3: Verificar Problema Antiguo
```
Usuario: !solved 4A
Bot: 🎉 Problema resuelto verificado! ✅ El problema `4A` ha sido marcado como resuelto.
```

## Integración con Otros Comandos

El comando `!solved` trabaja en conjunto con:

- **`!account`**: Requerido para la vinculación inicial
- **`!problem`**: Para obtener problemas nuevos que resolver
- Futuros comandos de estadísticas y progreso

## Solución de Problemas

### Mi problema está resuelto pero no se detecta

1. **Verifica el formato**: Asegúrate de usar el formato correcto (`contestID + index`)
2. **Confirma tu handle**: Revisa que tu cuenta esté bien vinculada con `!account`
3. **Perfil público**: Confirma que tu perfil de Codeforces sea público
4. **Veredicto**: Asegúrate de que la submission tenga veredicto "OK", no "Accepted" parcial
5. **Reintenta**: La API puede estar temporalmente indisponible

### El comando no responde

1. Verifica tu conexión a internet
2. La API de Codeforces puede estar en mantenimiento
3. Tu handle de Codeforces puede haber cambiado
4. Contacta a los administradores del servidor

## Seguridad y Privacidad

- 🔒 **Solo datos públicos**: Solo accede a información pública de tu perfil
- 👤 **Verificación personal**: Cada usuario solo puede marcar sus propios problemas
- 📝 **Registro local**: Los problemas resueltos se guardan localmente en el servidor
- 🚫 **No modificación**: El bot no modifica nada en tu cuenta de Codeforces

---

*Este comando te ayuda a llevar un registro preciso de tu progreso en competitive programming y participar en las actividades del servidor.*