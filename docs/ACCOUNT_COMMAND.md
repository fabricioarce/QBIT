# Comando !account

## Descripción

El comando `!account` permite vincular tu cuenta de **Codeforces** con tu perfil de Discord en el servidor. Una vez vinculada, el bot podrá rastrear tu progreso, estadísticas y personalizar la experiencia según tu nivel en Codeforces.

## Uso

```
!account <handle_de_codeforces>
```

### Parámetros

- `<handle_de_codeforces>`: Tu nombre de usuario (handle) en Codeforces

### Ejemplos

```
!account tourist
!account Errichto  
!account tu_usuario_cf
```

## Funcionalidad

Cuando ejecutas el comando, el bot:

1. **Valida el handle**: Verifica que hayas proporcionado un handle de Codeforces
2. **Consulta la API**: Se conecta a la API oficial de Codeforces para obtener tu información
3. **Valida la existencia**: Confirma que el usuario existe en Codeforces
4. **Guarda en la base de datos**: Almacena la siguiente información:
   - Handle de Codeforces
   - Rating actual
   - Rank actual
   - Rating máximo alcanzado
5. **Confirma la vinculación**: Muestra un mensaje de éxito con tu información

## Información Almacenada

El bot guarda los siguientes datos de tu perfil de Codeforces:

- **Handle**: Tu nombre de usuario en Codeforces
- **Rating**: Tu rating actual
- **Rank**: Tu clasificación actual (ej: "pupil", "specialist", "expert", etc.)
- **Rating Máximo**: El rating más alto que has alcanzado

## Respuestas del Bot

### Éxito ✅
```
✅ Cuenta de Codeforces vinculada exitosamente!

👤 Handle: `tu_usuario`
🏆 Rating: `1500`
🎯 Rank: `specialist`
📊 Rating Máximo: `1650`
```

### Errores ❌

**Handle no proporcionado:**
```
❌ Por favor proporciona un handle de Codeforces. Uso: `!account tu_handle`
```

**Usuario no encontrado:**
```
❌ No se encontró el usuario `handle_inexistente` en Codeforces. Verifica que el handle sea correcto.
```

**Error de API:**
```
❌ Error al conectar con la API de Codeforces. Intenta de nuevo más tarde.
```

**Error de base de datos:**
```
❌ Error al guardar la información en la base de datos. Intenta de nuevo más tarde.
```

## Notas Importantes

- ⚠️ **Actualización**: Si ya tienes una cuenta vinculada y ejecutas el comando nuevamente, se actualizará tu información con los datos más recientes
- 🔄 **Sincronización**: La información se obtiene en tiempo real desde Codeforces
- 📊 **Privacidad**: Solo se almacena información pública disponible en tu perfil de Codeforces
- 🌐 **Disponibilidad**: El comando depende de la disponibilidad de la API de Codeforces

## Comandos Relacionados

Una vez vinculada tu cuenta, podrás usar otros comandos que aprovechen esta información:

- `!problem <rating_min> <rating_max>`: Obtener problemas según tu nivel
- `!solved`: Ver estadísticas de problemas resueltos
- Y más funcionalidades personalizadas según tu progreso

## Soporte

Si experimentas problemas con el comando:

1. Verifica que el handle de Codeforces sea correcto
2. Asegúrate de que tu perfil de Codeforces sea público
3. Intenta nuevamente después de unos minutos si hay errores de API
4. Contacta a los administradores del servidor si el problema persiste

---

*Este comando es esencial para aprovechar al máximo las funcionalidades del bot relacionadas con competitive programming.*