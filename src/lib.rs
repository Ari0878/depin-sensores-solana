use anchor_lang::prelude::*;

declare_id!("8uRxqH4M2ajAnibpcWdxpVu8WvYFpko12kBdWfs9fSvB"); // Program ID

#[program]
pub mod depin_sensores {
    use super::*;

    /////////////////////////// Crear Red ///////////////////////////
    // Crea una nueva red DePIN, inicializa el owner y la lista vacía de sensores
    pub fn crear_red(context: Context<NuevaRed>, n_red: String) -> Result<()> {
        let owner_id = context.accounts.owner.key();
        let sensores = Vec::<Pubkey>::new();

        context.accounts.red.set_inner(Red {
            owner: owner_id,
            n_red: n_red.clone(),
            sensores,
        });

        msg!("Red DePIN '{}' creada exitosamente! Owner: {}", n_red, owner_id);
        Ok(())
    }

    /////////////////////////// Agregar Sensor ///////////////////////////
    // Agrega un nuevo sensor a la red, solo el owner puede hacerlo
    // Se valida que el caller sea el owner antes de guardar el sensor
    pub fn agregar_sensor(
        context: Context<NuevoSensor>,
        nombre: String,
        tipo_sensor: String,
        ubicacion: String,
    ) -> Result<()> {
        require!(
            context.accounts.red.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let sensor = Sensor {
            red: context.accounts.red.n_red.clone(),
            nombre: nombre.clone(),
            tipo_sensor: tipo_sensor.clone(),
            ubicacion: ubicacion.clone(),
            activo: true,
        };

        context.accounts.sensor.set_inner(sensor);
        context.accounts.red.sensores.push(context.accounts.sensor.key());

        msg!(
            "Sensor '{}' (tipo: {}) agregado a la red '{}'. Ubicacion: {}",
            nombre,
            tipo_sensor,
            context.accounts.red.n_red,
            ubicacion
        );
        Ok(())
    }

    /////////////////////////// Eliminar Sensor ///////////////////////////
    // Elimina un sensor de la red, solo el owner puede hacerlo
    // Se verifica que el sensor exista y pertenezca a la red antes de eliminarlo
    pub fn eliminar_sensor(context: Context<EliminarSensor>, nombre: String) -> Result<()> {
        require!(
            context.accounts.red.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let red = &mut context.accounts.red;

        require!(
            context.accounts.sensor.red == red.n_red,
            Errores::SensorNoPertenece
        );

        require!(
            red.sensores.contains(&context.accounts.sensor.key()),
            Errores::SensorNoExiste
        );

        let pos = red
            .sensores
            .iter()
            .position(|&x| x == context.accounts.sensor.key())
            .ok_or(Errores::SensorNoExiste)?;

        red.sensores.remove(pos);

        msg!(
            "Sensor '{}' eliminado de la red '{}'. Owner: {}",
            nombre,
            red.n_red,
            context.accounts.owner.key()
        );
        Ok(())
    }

    /////////////////////////// Alternar Estado ///////////////////////////
    // Cambia el estado del sensor (activo/inactivo), solo el owner puede hacerlo
    // Invierte el valor booleano actual del campo 'activo'
    pub fn alternar_estado(context: Context<ModificarSensor>, nombre: String) -> Result<()> {
        require!(
            context.accounts.red.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let sensor = &mut context.accounts.sensor;
        let nuevo_estado = !sensor.activo;
        sensor.activo = nuevo_estado;

        msg!(
            "Sensor '{}' ahora está: {}",
            nombre,
            if nuevo_estado { "ACTIVO" } else { "INACTIVO" }
        );
        Ok(())
    }
}

/////////////////////////// Errores ///////////////////////////
// Definición de errores personalizados para el programa
#[error_code]
pub enum Errores {
    #[msg("Error: no eres el propietario de esta red")]
    NoEresElOwner,
    #[msg("Error: el sensor no existe en esta red")]
    SensorNoExiste,
    #[msg("Error: el sensor no pertenece a esta red")]
    SensorNoPertenece,
}

/////////////////////////// CUENTAS ///////////////////////////

#[account]
#[derive(InitSpace)]
pub struct Red {
    pub owner: Pubkey,           // Dirección del propietario de la red

    #[max_len(60)]
    pub n_red: String,           // Nombre de la red

    #[max_len(10)]
    pub sensores: Vec<Pubkey>,   // Vector con las direcciones de los sensores pertenecientes a esta red
}

#[account]
#[derive(InitSpace)]
pub struct Sensor {
    #[max_len(60)]
    pub red: String,             // Nombre de la red a la que pertenece este sensor

    #[max_len(60)]
    pub nombre: String,          // Nombre del sensor

    #[max_len(30)]
    pub tipo_sensor: String,     // Tipo de sensor: "temperatura", "humedad", "presion"

    #[max_len(60)]
    pub ubicacion: String,       // Ubicación del sensor

    pub activo: bool,            // Estado actual del sensor (activo/inactivo)
}

/////////////////////////// CONTEXTOS ///////////////////////////

#[derive(Accounts)]
#[instruction(n_red: String)]
pub struct NuevaRed<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,    // Cuenta que firma y pagará la creación

    #[account(
        init,
        payer = owner,
        space = 8 + Red::INIT_SPACE,
        seeds = [b"red", n_red.as_bytes(), owner.key().as_ref()],
        bump
    )]
    pub red: Account<'info, Red>,  // Cuenta de la red a crear (PDA)

    pub system_program: Program<'info, System>,  // Programa del sistema para crear cuentas
}

#[derive(Accounts)]
#[instruction(nombre: String)]
pub struct NuevoSensor<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,    // Cuenta que firma y pagará la creación

    #[account(
        init,
        payer = owner,
        space = 8 + Sensor::INIT_SPACE,
        seeds = [b"sensor", nombre.as_bytes(), owner.key().as_ref()],
        bump
    )]
    pub sensor: Account<'info, Sensor>,  // Cuenta del sensor a crear (PDA)

    #[account(mut)]
    pub red: Account<'info, Red>,        // Cuenta de la red a la que se agregará el sensor

    pub system_program: Program<'info, System>,  // Programa del sistema para crear cuentas
}

#[derive(Accounts)]
pub struct ModificarSensor<'info> {
    pub owner: Signer<'info>,    // Cuenta que firma la transacción

    #[account(mut)]
    pub sensor: Account<'info, Sensor>,  // Cuenta del sensor a modificar

    #[account(mut)]
    pub red: Account<'info, Red>,        // Cuenta de la red dueña del sensor
}

#[derive(Accounts)]
pub struct EliminarSensor<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,    // Cuenta que firma y recibirá el SOL de la cuenta cerrada

    #[account(
        mut,
        close = owner,
        constraint = sensor.red == red.n_red @ Errores::SensorNoPertenece
    )]
    pub sensor: Account<'info, Sensor>,  // Cuenta del sensor a eliminar (se cierra automáticamente)

    #[account(mut)]
    pub red: Account<'info, Red>,        // Cuenta de la red de la que se eliminará el sensor
}
