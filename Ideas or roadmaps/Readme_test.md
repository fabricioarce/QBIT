# 🚀 Roadmap - Bot de Programación Competitiva para Discord

## 📋 Información del Proyecto

**Objetivo:** Bot de Discord para comunidad de programación competitiva enfocado en Codeforces, con sistema de gamificación para fomentar la participación y el aprendizaje colaborativo.

**Tecnologías:** Rust + Serenity + PostgreSQL + Codeforces API

---

## 🎯 MVP (Minimum Viable Product)

### Funcionalidades核心 del MVP

El MVP debe responder a la pregunta: **"¿Qué necesito para que la comunidad empiece a resolver problemas juntos YA?"**

#### ✅ Debe Incluir:

1. **Sistema de Problemas Básico**
   - Comando para obtener un problema aleatorio de Codeforces
   - Comando para registrar cuando alguien resuelve un problema
   - Visualización simple de quién ha resuelto qué

2. **Tracking Mínimo**
   - Base de datos con usuarios y problemas resueltos
   - Comando para ver tu propio progreso
   - Ranking simple de la comunidad

3. **Problema Diario**
   - Un problema automático cada día
   - Canal dedicado donde se publica
   - Motivación para resolver en comunidad

#### ❌ NO Incluir en MVP:

- Sistema de puntos/economía (viene después)
- Tienda (viene después)
- Sistema de strikes/moderación complejo
- Estadísticas avanzadas
- Sistema de bienvenida elaborado

### ⏱️ Tiempo Estimado MVP: 2-3 semanas

---

## 🗺️ Roadmap Detallado

### **Fase 0: Preparación** (3-5 días)

**Objetivo:** Tener el entorno listo y entender Rust básico

- [ ] Completar capítulos 1-8 de "The Rust Book"
- [ ] Hacer ejercicios de Rustlings hasta ownership
- [ ] Configurar entorno de desarrollo
- [ ] Crear proyecto base con la estructura modular
- [ ] Bot "Hello World" funcionando en Discord
- [ ] Conexión exitosa a PostgreSQL

**Entregable:** Bot que responde `!ping` → `Pong!`

---

### **Fase 1: Core de Codeforces** (1 semana) 🔴 PRIORIDAD MÁXIMA

**Objetivo:** Que la gente pueda empezar a resolver problemas

#### Sprint 1.1: Cliente Codeforces (2-3 días)

- [ ] Implementar cliente HTTP para Codeforces API
- [ ] Struct `Problem` con todos los campos relevantes
- [ ] Función `get_problemset()` - obtener todos los problemas
- [ ] Función `get_problem_by_id(contest_id, index)` - problema específico
- [ ] Cache en memoria de problemas (para no saturar API)
- [ ] Manejo de errores de API (rate limits, timeouts)

**Entregable:** Cliente funcional que puede obtener problemas

#### Sprint 1.2: Comando !problema (2 días)

- [ ] Comando `!problema [dificultad]` - problema aleatorio
  - Sin argumentos: cualquier problema
  - Con dificultad: filtrar por rating (800, 1200, 1500, etc.)
- [ ] Formato bonito con Discord Embed:
  - Nombre del problema
  - Dificultad (rating)
  - Tags
  - Link directo al problema
- [ ] Sistema de filtros adicionales (opcional):
  - `!problema 1500 graph` - problema de grafos de 1500
  - `!problema easy` - alias para rangos (easy=800-1200)

**Entregable:** Usuarios pueden pedir problemas y los reciben formateados

#### Sprint 1.3: Base de Datos - Tracking Básico (2 días)

- [ ] Diseñar schema de PostgreSQL:
  ```sql
  users (discord_id, codeforces_handle, created_at)
  solved_problems (user_id, problem_id, solved_at)
  daily_problems (date, problem_id, channel_id)
  ```
- [ ] Migraciones SQL
- [ ] Queries básicas (get_user, create_user, add_solved)
- [ ] Comando `!vincular <handle>` - conectar Discord con Codeforces
- [ ] Comando `!solved <problem_id>` - marcar problema resuelto
  - Validación: verificar que el problema existe
  - Prevenir duplicados
  - Confirmación visual

**Entregable:** Sistema de persistencia funcionando

---

### **Fase 2: Engagement Básico** (4-5 días) 🟡 ALTA PRIORIDAD

**Objetivo:** Dar razones para que la gente vuelva diariamente

#### Sprint 2.1: Problema Diario (2 días)

- [ ] Scheduler con `tokio-cron-scheduler`
- [ ] Tarea diaria (ej: 9:00 AM hora del server)
- [ ] Selección inteligente de problema:
  - Rotación de dificultades (fácil → medio → difícil)
  - Evitar repetir tags consecutivos
  - Balance de temas (DP, graphs, math, etc.)
- [ ] Publicación automática en canal dedicado
- [ ] Embed especial "🌟 Problema del Día"
- [ ] Mención de rol @ProblemSolvers (opcional)

**Entregable:** Problema nuevo cada día automáticamente

#### Sprint 2.2: Leaderboard y Stats (2-3 días)

- [ ] Comando `!ranking [periodo]` - top resolvedores
  - Semanal (por defecto)
  - Mensual
  - Todo el tiempo
- [ ] Comando `!yo` o `!stats` - tus propias estadísticas:
  - Problemas resueltos (total y por dificultad)
  - Racha actual (días consecutivos)
  - Posición en ranking
  - Tags más resueltos
- [ ] Comando `!stats @usuario` - ver stats de alguien más
- [ ] Visualización con embeds coloridos

**Entregable:** Competencia amigable y visibilidad de progreso

---

### **🎉 PUNTO DE LANZAMIENTO MVP** 

**En este punto puedes lanzar el bot a la comunidad**

---

### **Fase 3: Gamificación** (1 semana) 🟢 MEDIA PRIORIDAD

**Objetivo:** Sistema de recompensas para mantener engagement

#### Sprint 3.1: Sistema de Puntos (3 días)

- [ ] Añadir columna `points` a tabla users
- [ ] Sistema de recompensas por resolver:
  - Rating < 1000: 10 puntos
  - Rating 1000-1400: 20 puntos
  - Rating 1400-1800: 35 puntos
  - Rating 1800+: 50 puntos
  - Bonus problema diario: +50%
  - Primera vez resolviendo ese tag: +20%
- [ ] Comando `!puntos` - ver tus puntos
- [ ] Actualizar `!ranking` para incluir puntos
- [ ] Sistema de niveles (opcional):
  - Novato: 0-500 pts
  - Aprendiz: 500-2000 pts
  - Competidor: 2000-5000 pts
  - Experto: 5000+ pts

**Entregable:** Sistema de puntos funcionando

#### Sprint 3.2: Tienda Básica (2-3 días)

- [ ] Tabla `shop_items` y `purchases`
- [ ] Comando `!tienda` - ver items disponibles
- [ ] Comando `!comprar <item>` - comprar con puntos
- [ ] Items iniciales sugeridos:
  - **Roles especiales** (100-500 pts)
    - "🔥 On Fire" (visible en lista de miembros)
    - "🎯 Problem Hunter"
    - Colores personalizados
  - **Hint del problema diario** (50 pts)
  - **Problema personalizado** (200 pts)
    - El bot te sugiere uno perfecto para tu nivel
  - **Destacar en ranking por 1 semana** (300 pts)
  - **Saltear problema diario sin perder racha** (150 pts)

**Entregable:** Tienda con items comprables

#### Sprint 3.3: Rachas y Logros (2 días)

- [ ] Sistema de rachas diarias
- [ ] Badges/logros desbloqueables:
  - "Primera Sangre" - primer problema resuelto
  - "Racha de Fuego" - 7 días consecutivos
  - "Centenario" - 100 problemas resueltos
  - "Especialista en X" - 20 problemas de un tag
  - "Madrugador" - resolver problema del día antes de 12pm
- [ ] Comando `!logros` - ver tus badges
- [ ] Mostrar badges en `!stats`

**Entregable:** Sistema de logros motivacional

---

### **Fase 4: Comunidad y Colaboración** (1 semana) 🔵 MEDIA PRIORIDAD

**Objetivo:** Fomentar ayuda mutua

#### Sprint 4.1: Sistema de Ayuda (3 días)

- [ ] Comando `!ayuda <problem_id>` - pedir ayuda en un problema
  - Crea thread automático
  - Mención a rol @Helpers
  - Embed con link al problema
- [ ] Sistema de reputación para helpers:
  - Reacciones ✅ de quien pidió ayuda = +1 rep
  - Comando `!agradecer @usuario` = +5 puntos al helper
- [ ] Comando `!helpers` - top personas que ayudan
- [ ] Canal de "Dudas Resueltas" donde se archivan buenos threads

**Entregable:** Sistema de Q&A integrado

#### Sprint 4.2: Sesiones de Estudio Grupales (2 días)

- [ ] Comando `!sesion crear <tema> <hora>` 
  - Crea evento en Discord
  - Notifica a interesados
- [ ] Comando `!sesion unirse <id>`
- [ ] Comando `!sesiones` - ver próximas sesiones
- [ ] Bot postea recordatorio 30 min antes

**Entregable:** Coordinación de estudio grupal

#### Sprint 4.3: Compartir Soluciones (2 días)

- [ ] Comando `!solucion <problem_id> <link>` 
  - Compartir tu código (link a pastebin/github)
  - Opcional: explicación
- [ ] Sistema de review de código:
  - Otros pueden reaccionar y comentar
  - Upvotes para mejores soluciones
- [ ] Comando `!soluciones <problem_id>` - ver todas las soluciones compartidas

**Entregable:** Biblioteca de soluciones comunitaria

---

### **Fase 5: Moderación y Gestión** (4-5 días) ⚪ BAJA PRIORIDAD

**Objetivo:** Herramientas para mods y admins

#### Sprint 5.1: Sistema de Strikes (2 días)

- [ ] Tabla `strikes` en DB
- [ ] Comandos de moderación:
  - `!warn @usuario <razon>` - dar strike
  - `!strikes @usuario` - ver strikes
  - `!clearstrikes @usuario` - limpiar strikes
- [ ] Auto-acciones:
  - 3 strikes = timeout 24h
  - 5 strikes = kick
  - 7 strikes = ban
- [ ] Log de moderación en canal privado

**Entregable:** Sistema de moderación básico

#### Sprint 5.2: Sistema de Bienvenida (1 día)

- [ ] Mensaje automático al unirse alguien
- [ ] Embed con:
  - Bienvenida personalizada
  - Guía rápida del servidor
  - Comando `!vincular` para empezar
  - Roles disponibles
- [ ] Auto-asignar rol "Nuevo" (se quita al vincular CF)

**Entregable:** Onboarding automático

#### Sprint 5.3: Admin Tools (1-2 días)

- [ ] `!dar_puntos @usuario <cantidad>` - dar puntos manual
- [ ] `!resetear_usuario @usuario` - resetear progreso
- [ ] `!stats_server` - estadísticas del servidor:
  - Total usuarios activos
  - Problemas resueltos esta semana
  - Dificultad promedio
- [ ] `!anuncio <mensaje>` - anuncio bonito en canal principal

**Entregable:** Herramientas de administración

---

## 📊 Métricas de Éxito

### Semana 1-2 (MVP):
- ✅ 10+ usuarios vinculan su Codeforces
- ✅ 50+ problemas resueltos colectivamente
- ✅ Problema diario con al menos 3 personas intentándolo

### Mes 1 (Post-Gamificación):
- ✅ 30+ usuarios activos semanalmente
- ✅ 200+ problemas resueltos
- ✅ 5+ personas con rachas de 7+ días

### Mes 2-3 (Post-Comunidad):
- ✅ 50+ usuarios activos
- ✅ 10+ sesiones de estudio organizadas
- ✅ 100+ interacciones de ayuda mutua

---

## 🎨 Features Futuras (Post-Roadmap)

Ideas para después de completar el roadmap:

- **Competencias Internas** - Contests privados del server
- **Sistema de Mentorías** - Parear novatos con expertos
- **Integración con otros judges** - AtCoder, LeetCode, etc.
- **Análisis de progreso con IA** - Sugerencias personalizadas
- **Integración con voice channels** - "Coding together" sessions
- **Newsletter semanal** - Resumen de actividad del server
- **Sistema de equipos** - Competencia por equipos

---

## 💡 Tips de Implementación

### Priorización
1. **Codeforces primero, siempre** - Es el core value del bot
2. **Engagement sobre features** - Mejor 3 features que enganchan que 10 mediocres
3. **Comunidad sobre automatización** - Si algo fomenta interacción humana, priorizalo

### Testing en Producción
- Lanza MVP rápido con grupo beta pequeño (5-10 personas)
- Itera basándote en feedback REAL
- No construyas features que nadie pidió

### Mantén Simple
- Menos comandos bien hechos > muchos comandos mediocres
- UI clara > features escondidas
- Mensajes concisos > explicaciones largas

---

**¿Listo para empezar? Comienza con la Fase 0 y construye el MVP en 2-3 semanas. ¡Suerte! 🚀**