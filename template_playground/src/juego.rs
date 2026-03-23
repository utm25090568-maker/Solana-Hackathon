use anchor_lang::prelude::*;

declare_id!("Tu_ID_de_Programa_Aqui"); // Esto se genera al hacer 'anchor build'

#[program]
pub mod juego_adivinanza {
    use super::*;

    // Esta función inicia el juego
    pub fn iniciar_juego(ctx: Context<Iniciar>, numero_secreto: u32) -> Result<()> {
        let juego = &mut ctx.accounts.juego;
        juego.numero_secreto = numero_secreto;
        juego.intentos = 0;
        juego.jugador = *ctx.accounts.user.key;
        msg!("¡Juego iniciado! El número ha sido guardado en la blockchain.");
        Ok(())
    }

    // Esta función es para que el usuario intente adivinar
    pub fn adivinar(ctx: Context<Adivinar>, suposicion: u32) -> Result<()> {
        let juego = &mut ctx.accounts.juego;
        juego.intentos += 1;

        if suposicion < juego.numero_secreto {
            msg!(">> Muy bajo. Intenta de nuevo.");
        } else if suposicion > juego.numero_secreto {
            msg!(">> Muy alto. Intenta de nuevo.");
        } else {
            msg!("¡FELICIDADES! Adivinaste en {} intentos.", juego.intentos);
        }
        Ok(())
    }
}

#[account]
pub struct Juego {
    pub numero_secreto: u32,
    pub intentos: u32,
    pub jugador: Pubkey,
}

#[derive(Accounts)]
pub struct Iniciar<'info> {
    #[account(init, payer = user, space = 8 + 4 + 4 + 32)]
    pub juego: Account<'info, Juego>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Adivinar<'info> {
    #[account(mut)]
    pub juego: Account<'info, Juego>,
}