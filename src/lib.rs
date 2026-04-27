use anchor_lang::prelude::*;

declare_id!("HVm4phF68SP6xAmATUPGzdjqz68fQrMB8KqbcpQFGWMu");

#[program]
pub mod videocentro_db {
    use super::*;

    pub fn mi_primer_instruccion(_ctx: Context<Saludo>) -> Result<()> {
        msg!("Mi instruccion, funciona !!! :D");
        Ok(())
    }

    pub fn crear_videocentro(ctx: Context<CrearVideocentro>, nombre: String) -> Result<()> {
        let videocentro = &mut ctx.accounts.videocentro;
        videocentro.nombre = nombre;
        videocentro.owner = *ctx.accounts.owner.key;
        videocentro.peliculas = Vec::new();
        Ok(())
    }

    pub fn agregar_pelicula(
        ctx: Context<AgregarPelicula>,
        genero: String,
        titulo: String,
        nombre_cliente: String,  
        anio_estreno: u16,
        director: String,
        
    ) -> Result<()> {
        let videocentro = &mut ctx.accounts.videocentro;

        let nueva_pelicula = Pelicula {
            genero,
            titulo,
            nombre_cliente,  
            anio_estreno,
            disponible: true,
            director,
          
        };

        videocentro.peliculas.push(nueva_pelicula);
        Ok(())
    }

    pub fn eliminar_registro_pelicula(
        ctx: Context<EliminarRegistroPelicula>,
        titulo_pelicula: String,
    ) -> Result<()> {
        let videocentro = &mut ctx.accounts.videocentro;

        if let Some(pos) = videocentro
            .peliculas
            .iter()
            .position(|p| p.titulo == titulo_pelicula)
        {
            videocentro.peliculas.swap_remove(pos);
            Ok(())
        } else {
            Err(ErrorCode::PeliculaNoEncontrada.into())
        }
    }

    pub fn ver_peliculas(ctx: Context<VerPeliculas>) -> Result<()> {
        let videocentro = &ctx.accounts.videocentro;

        if videocentro.peliculas.is_empty() {
            msg!("No hay películas registradas en el Videocentro.");
            return Ok(());
        }

        msg!(
            "Listado de películas en el Videocentro '{}':",
            videocentro.nombre
        );

        for (i, pelicula) in videocentro.peliculas.iter().enumerate() {
            msg!(
                "Película #{}: Título: {}, Género: {}, Nombre Cliente: {}, Año: {}, Disponible: {}, Director: {}",
                i + 1,
                pelicula.titulo,
                pelicula.genero,
                pelicula.nombre_cliente,  
                pelicula.anio_estreno,
                pelicula.disponible,
                pelicula.director
                
            );
        }

        Ok(())
    }

    pub fn cambiar_disponibilidad(ctx: Context<CambiarDisponibilidad>, titulo_pelicula: String) -> Result<()> {
        let videocentro = &mut ctx.accounts.videocentro;

        if let Some(pelicula) = videocentro
            .peliculas
            .iter_mut()
            .find(|p| p.titulo == titulo_pelicula)
        {
            pelicula.disponible = !pelicula.disponible;
            msg!(
                "Disponibilidad actualizada: la película '{}' ahora está {}.",
                titulo_pelicula,
                if pelicula.disponible { "disponible" } else { "no disponible" }
            );
            Ok(())
        } else {
            Err(ErrorCode::PeliculaNoEncontrada.into())
        }
    }
}

#[error_code]
pub enum ErrorCode {
    #[msg("Película no encontrada en el Videocentro.")]
    PeliculaNoEncontrada,
}

#[derive(Accounts)]
pub struct Saludo {}

#[derive(Accounts)]
pub struct CrearVideocentro<'info> {
    #[account(
        init, 
        payer = owner, 
        space = 8 + Videocentro::INIT_SPACE,
        seeds = [b"videocentro", owner.key().as_ref()],
        bump
    )]
    pub videocentro: Account<'info, Videocentro>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct Videocentro {
    #[max_len(100)]
    pub nombre: String,
    pub owner: Pubkey,
    #[max_len(10)]
    pub peliculas: Vec<Pelicula>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct Pelicula {
    #[max_len(60)]
    pub genero: String,

    #[max_len(60)]
    pub titulo: String,

    #[max_len(60)]
    pub nombre_cliente: String,  

    pub anio_estreno: u16,
    pub disponible: bool,

    #[max_len(60)]
    pub director: String,
    
}

#[derive(Accounts)]
pub struct AgregarPelicula<'info> {
    #[account(mut)]
    pub videocentro: Account<'info, Videocentro>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct EliminarRegistroPelicula<'info> {
    #[account(mut)]
    pub videocentro: Account<'info, Videocentro>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct VerPeliculas<'info> {
    pub videocentro: Account<'info, Videocentro>,
}

#[derive(Accounts)]
pub struct CambiarDisponibilidad<'info> {
    #[account(mut)]
    pub videocentro: Account<'info, Videocentro>,
    pub owner: Signer<'info>,
}
