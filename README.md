### Guía de uso de la API REST

#### Obtener todas las tareas (GET)
```http
GET /tasks
```
- Descripción: Devuelve una lista de todas las tareas.
- Respuesta: JSON con las tareas.

#### Añadir una nueva tarea (POST)
```http
POST /tasks
Content-Type: application/json

{
    "description": "Nueva tarea"
}
```
- Descripción: Añade una nueva tarea.
- Respuesta: JSON con la tarea añadida.

#### Marcar una tarea como completada (PUT)
```http
PUT /tasks/{id}/mark-done
```
- Descripción: Marca la tarea con el ID especificado como completada.

#### Desmarcar una tarea como completada (PUT)
```http
PUT /tasks/{id}/unmark-done
```
- Descripción: Desmarca la tarea con el ID especificado como completada.
- Respuesta: JSON con la tarea modificada.

#### Eliminar una tarea por ID (DELETE)
```http
DELETE /tasks/{id}
```
- Descripción: Elimina la tarea con el ID especificado.
- Respuesta: Código de estado HTTP 204 (No Content).