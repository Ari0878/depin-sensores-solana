import { PublicKey } from "@solana/web3.js";

////////////////// Constantes ////////////////////
// Nombre fijo de la red que vamos a crear/manejar
const n_red = "RedSensoresMX";
// Owner es la wallet conectada actualmente en el playground
const owner = pg.wallet.publicKey;

console.log("Mi address:", owner.toString());
const balance = await pg.connection.getBalance(owner);
console.log(`Mi balance: ${balance / web3.LAMPORTS_PER_SOL} SOL`);

////////////////// PDAs //////////////////
// Calcula la PDA (dirección determinística) de la red basada en su nombre y el owner
function pdaRed(n_red: string) {
  return PublicKey.findProgramAddressSync(
    [
      Buffer.from("red"),           // Semilla: "red"
      Buffer.from(n_red),           // Semilla: nombre de la red
      owner.toBuffer(),             // Semilla: dirección del owner
    ],
    pg.PROGRAM_ID
  );
}

// Calcula la PDA de un sensor basada en su nombre y el owner
function pdaSensor(n_sensor: string) {
  return PublicKey.findProgramAddressSync(
    [
      Buffer.from("sensor"),        // Semilla: "sensor"
      Buffer.from(n_sensor),        // Semilla: nombre del sensor
      owner.toBuffer(),             // Semilla: dirección del owner
    ],
    pg.PROGRAM_ID
  );
}

////////////////// Crear Red //////////////////
// Función para crear una nueva red en el programa
async function crearRed(n_red: string) {
  const [pda_red] = pdaRed(n_red);  // Obtengo la PDA donde se guardará la red

  const txHash = await pg.program.methods
    .crearRed(n_red)                 // Llamo a la instrucción del programa
    .accounts({
      owner,                         // Cuenta que firma y paga
      red: pda_red,                  // Cuenta nueva de la red
    })
    .rpc();

  console.log("Red creada! txHash:", txHash);
}

////////////////// Agregar Sensor //////////////////
// Función para agregar un nuevo sensor a la red existente
async function agregarSensor(
  n_sensor: string,
  tipo: string,
  ubicacion: string
) {
  const [pda_sensor] = pdaSensor(n_sensor);  // PDA del nuevo sensor
  const [pda_red] = pdaRed(n_red);           // PDA de la red existente

  const txHash = await pg.program.methods
    .agregarSensor(n_sensor, tipo, ubicacion)  // Parámetros: nombre, tipo, ubicación
    .accounts({
      owner,                                    // Owner firma la tx
      sensor: pda_sensor,                       // Cuenta nueva del sensor
      red: pda_red,                             // Cuenta de la red (se modificará)
    })
    .rpc();

  console.log("Sensor agregado! txHash:", txHash);
}

////////////////// Alternar Estado //////////////////
// Cambia el estado de un sensor (activo <-> inactivo)
async function alternarEstado(n_sensor: string) {
  const [pda_sensor] = pdaSensor(n_sensor);  // PDA del sensor a modificar
  const [pda_red] = pdaRed(n_red);           // PDA de su red

  const txHash = await pg.program.methods
    .alternarEstado(n_sensor)                 // Parámetro: nombre del sensor
    .accounts({
      owner,                                  // Owner firma (solo él puede hacerlo)
      sensor: pda_sensor,                     // Cuenta del sensor (se modifica)
      red: pda_red,                           // Cuenta de la red (para verificar owner)
    })
    .rpc();

  console.log("Estado alternado! txHash:", txHash);
}

////////////////// Eliminar Sensor //////////////////
// Elimina completamente un sensor (cierra su cuenta y recupera SOL)
async function eliminarSensor(n_sensor: string) {
  const [pda_sensor] = pdaSensor(n_sensor);  // PDA del sensor a eliminar
  const [pda_red] = pdaRed(n_red);           // PDA de su red

  const txHash = await pg.program.methods
    .eliminarSensor(n_sensor)                 // Parámetro: nombre del sensor
    .accounts({
      owner,                                  // Owner firma y recibe el SOL de la cuenta cerrada
      sensor: pda_sensor,                     // Cuenta del sensor (se cerrará)
      red: pda_red,                           // Cuenta de la red (se actualizará la lista)
    })
    .rpc();

  console.log("Sensor eliminado! txHash:", txHash);
}

////////////////// Ver Sensores //////////////////
// Función para consultar y mostrar todos los sensores de una red
async function verSensores(n_red: string) {
  const [pda_red] = pdaRed(n_red);  // Obtengo la PDA de la red

  try {
    // Fetch de la cuenta de la red desde la blockchain
    const redAccount = await pg.program.account.red.fetch(pda_red);
    const total = redAccount.sensores.length;  // Cantidad de sensores en la red

    if (total === 0) {
      console.log("La red no tiene sensores registrados.");
      return;
    }

    console.log(`Red: ${redAccount.nRed} | Total sensores: ${total}`);

    // Itero sobre cada sensor y muestro sus datos
    for (let i = 0; i < total; i++) {
      const sensorKey = redAccount.sensores[i];      // PDA del sensor
      const s = await pg.program.account.sensor.fetch(sensorKey);  // Datos del sensor

      console.log(
        `Sensor #${i + 1}:
  * Nombre:    ${s.nombre}
  * Tipo:      ${s.tipoSensor}
  * Ubicacion: ${s.ubicacion}
  * Activo:    ${s.activo}
  * PDA:       ${sensorKey.toBase58()}`
      );
    }
  } catch (error) {
    console.error("Error al obtener sensores:", error);
  }
}

// ─── Prueba aquí ────────────────────────────────────────────
// crearRed(n_red);
// agregarSensor("SensorTemp01", "temperatura", "Monterrey, MX - Lat:25.6 Lon:-100.3");
// agregarSensor("SensorHum01", "humedad", "CDMX - Lat:19.4 Lon:-99.1");
// alternarEstado("SensorTemp01");
// eliminarSensor("SensorHum01");
// verSensores(n_red);
