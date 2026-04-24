# DePIN Sensores IoT — Solana + Anchor

Sistema descentralizado para el registro y gestión de sensores IoT en la blockchain de Solana. Implementa un modelo CRUD completo utilizando PDAs (Program Derived Addresses) para garantizar direcciones determinísticas y seguras.

## Que hace?

Permite a un propietario crear y administrar una red de sensores físicos (temperatura, humedad, presion, etc.) directamente en la blockchain de Solana. Cada sensor tiene metadata asociada (nombre, tipo, ubicacion, estado activo/inactivo) y el propietario tiene control total sobre su red.

## Como funciona el codigo

### Estructura de datos principal

El programa define dos cuentas principales:

**Red**
- Almacena la direccion del propietario
- Guarda el nombre de la red
- Mantiene una lista de direcciones (Pubkeys) de los sensores pertenecientes a esa red

**Sensor**
- Contiene el nombre de la red a la que pertenece
- Guarda su nombre identificador
- Almacena el tipo de sensor (temperatura, humedad, etc.)
- Registra su ubicacion geografica
- Mantiene un estado booleano (activo/inactivo)

### Seguridad y control de acceso

Cada red tiene un unico propietario. Las instrucciones que modifican la red o sus sensores verifican que quien firma la transaccion sea efectivamente el owner de la red. Esto se logra con la linea:
`require!(context.accounts.red.owner == context.accounts.owner.key(), Errores::NoEresElOwner)`

### PDAs (Program Derived Addresses)

Las PDAs son direcciones deterministicas que garantizan unicidad y seguridad:

**PDA de la red:** 
Se calcula con las semillas: ["red", nombre_red, direccion_del_owner]
Esto asegura que cada combinacion de owner + nombre de red genere una direccion unica.

**PDA del sensor:**
Se calcula con las semillas: ["sensor", nombre_sensor, direccion_del_owner]
Similar a la red, garantiza que cada sensor tenga una direccion predecible.

**Ventaja de usar PDAs:**
- Se puede encontrar la direccion de una cuenta sin consultar la blockchain
- Las cuentas no pueden ser creadas por cualquiera, solo mediante el programa
- Las semillas actuan como una especie de "primary key" compuesta

### Flujo de las instrucciones

**Crear red:**
1. Se calcula la PDA de la red usando las semillas
2. Se inicializa una nueva cuenta con espacio suficiente
3. Se guarda el owner, el nombre y una lista vacia de sensores

**Agregar sensor:**
1. Se verifica que el caller sea el owner de la red
2. Se calcula la PDA del sensor con sus semillas
3. Se crea la cuenta del sensor con sus datos
4. Se agrega la direccion del sensor al vector de la red

**Alternar estado:**
1. Se verifica que el caller sea el owner
2. Se invierte el booleano `activo` del sensor (true -> false, false -> true)

**Eliminar sensor:**
1. Se verifica que el caller sea el owner
2. Se comprueba que el sensor pertenezca a la red
3. Se busca y elimina la direccion del vector de sensores
4. Se cierra la cuenta del sensor (recuperando SOL para el owner)

### Manejo de errores

El programa define tres errores personalizados:
- `NoEresElOwner`: Cuando alguien que no es propietario intenta modificar la red
- `SensorNoExiste`: Cuando se busca un sensor que no esta en la lista
- `SensorNoPertenece`: Cuando se intenta eliminar un sensor de una red que no es la suya

## Instrucciones (CRUD)

| Instruccion        | Descripcion                                     | Quien puede ejecutarla |
|--------------------|-------------------------------------------------|------------------------|
| `crear_red`        | Crea una nueva red de sensores (PDA)           | Cualquiera             |
| `agregar_sensor`   | Registra un nuevo sensor en la red              | Solo el owner          |
| `alternar_estado`  | Cambia el sensor entre ACTIVO/INACTIVO         | Solo el owner          |
| `eliminar_sensor`  | Elimina un sensor y cierra su cuenta            | Solo el owner          |

## PDAs (resumen tecnico)

- **Red:** seeds = ["red", nombre_red, owner]
- **Sensor:** seeds = ["sensor", nombre_sensor, owner]

Ambas usan `owner.toBuffer()` como parte de las semillas para vincularlas al propietario.

## Requisitos previos

- Node.js instalado
- Wallet de Solana (Phantom, Solflare, etc.)
- SOL en la wallet para pagar las transacciones

## Como usar (Solana Playground)

1. Abrir https://beta.solpg.io/
2. Crear un nuevo proyecto Anchor
3. Pegar el codigo `src/lib.rs` (programa Rust)
4. Hacer click en Build y luego Deploy
5. En la pestaña client, pegar el codigo `client.ts`
6. Descomentar las funciones de prueba en orden logico
7. Ejecutar con el boton Run

## Orden recomendado para probar

```typescript
crearRed(n_red);                                    // Paso 1: crear red
agregarSensor("SensorTemp01", "temperatura", "ubicacion"); // Paso 2: agregar sensor
verSensores(n_red);                                // Paso 3: verificar
alternarEstado("SensorTemp01");                    // Paso 4: probar cambio de estado
eliminarSensor("SensorTemp01");                    // Paso 5: eliminar sensor
verSensores(n_red);                                // Paso 6: confirmar eliminacion
