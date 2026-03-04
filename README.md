# Primeros pasos en Solana
![Banner](./images/BANNERHACKATHON.png)

Solana es una blockchain de capa 1, es decir, cuenta con su propia infraestructura y no depende de otras blockchains para funcionar. Se encuentra orientada al alto rendimiento, y fue creada para soportar aplicaciones descentralizadas a gran escala con costos mínimos y confirmaciones casi inmediatas. Su diseño prioriza la eficiencia en la ejecución y la paralelización de transacciones.

Rust es el lenguaje principal para desarrollar programas en Solana. A través de él se implementa la lógica on-chain utilizando el modelo de cuentas y programas de la red, permitiendo construir contratos inteligentes seguros, eficientes y altamente optimizables.

Para facilitar el desarrollo en Rust sobre Solana existe Anchor, un framework que simplifica enormemente la creación de programas on-chain. Anchor proporciona:

* Un sistema de validación automática de cuentas mediante macros.
* Manejo simplificado de serialización y deserialización de datos.
* Gestión de PDAs (Program Derived Addresses) de forma declarativa.
* Generación automática de IDL (Interface Definition Language) para facilitar la interacción desde el frontend.
* Un entorno de testing más sencillo y estructurado.

Anchor, nos permite enfocarnos en la lógica del programa en lugar de manejar manualmente detalles de bajo nivel como validaciones repetitivas, manejo de bytes o verificación de firmas. Esto mejora la seguridad, reduce errores comunes y acelera el proceso de desarrollo.

# Entornos de desarollo
Hemos preparado el siguiente repositorio para que comiences a trabajar lo antes posible en tu proyecto si la necesidad de instalar nada de forma local!. Para ello, te porporcionamos las siguientes alternativas:

* Uso de Codespaces 
* Uso de Solana Playground

## Codespaces (Github)
Puedes comenzar dándole Fork a este repositorio (abajo te explicamos cómo 👇)

![fork](./images/fork.png)

* Puedes renombrar el repositorio a lo que sea que se ajuste con tu proyecto.
* Asegúrate de clonar este repositorio a tu cuenta usando el botón **`Fork`**.
* Presiona el botón **`<> Code`** y luego haz click en la sección **`Codespaces`**

    ![codespaces](./images/codespaces.png)

Por último, presiona **`Create codespace on master`**. Esto abrirá el proyecto en una interfaz gráfica de Visual Studio Code e instalará todas las herramientas necesarias para empezar a programar (es muy importante esperar a que este proceso termine):

![instalacion](./images/Instalacion.png)

El proceso de instalación finaliza cuando la terminal se reinicia y queda de la siguiente manera:

![fin](images/fin.png)

## Solana Playground
Solana Playground es un entorno de desarrollo online que permite escribir, compilar, desplegar y probar programas de Solana directamente desde el navegador, sin necesidad de instalar herramientas locales como Rust, Solana CLI o Anchor.

![Playground](./images/playground.png)

Para abrir el **Playground** solo es necesario dar clic 👉 [Aquí](https://beta.solpg.io)

## Configuración del entorno

Primero conectaremos el entorno con la devnet, lo que tambien procederá a la creación de una wallet. Para eso daremos clic en donde dice **Not Conected**:

![playground1](./images/playground1.png)

Saldrá la siguiente ventana donde daremos en el botón **Continue**:

![wallet](./images/wallet.png)

Como resultado se mostrará la siguiente información:

![status](./images/status.png)

* En verde: el estado de la conexión y el entorno al que se encuentra conectado

* En amarillo: la la dirección de la wallet conectada

* En azul: la cantidad de tokens en la wallet

> ℹ️ ¿Quieres ver el ejemplo de un "Hola Mundo" en Solana?. Da clic aquí: 👉 [Ver Ejemplo](./build-deploy/README.md)

## ¿Listo para empezar?

El primer paso es hacer `fork` al repositorio. Ya con el repositorio en tu cuenta lo siguiente que debes hacer es entrar a la carpeta `proyecto` y obtener el `permalink`:

![permalink](./images/permalink.png)

El cual uniremos con el siguiente enlace en nuestro navegador de preferencia:

```url
https://beta.solpg.io/
```

Lo que nos dará algo parecido a:

![url](./images/url.png)

Al pulsar enter seremos enviados al `Solana Playground` con nuestro proyecto abierto:

![pg](./images/pg.png)

Para guardarlo solo damos clic en el boton `import` y asignamos un nombre:

![import](./images/import.png)

## ¿Como actualizo mi repositorio?

Una vez que realices cambios o termines tu proyecto, es necesario que **copies todo el código**, ya con el código en el portapapeles nos dirigimos nuevamente a la carpeta proyecto de tu repositorio de github **donde se obtuvo el `permalink`**, donde entraremos al carpeta `src` y al archivo `lib.rs`:

![edit](./images/edit.png)

En `lib.rs` presionaremos el ícono en forma de lapiz (esquina superior derecha de la imagen 👆)

Nuevamente seleccionamos todo el código pero ahora presionamos `ctrl + v` para pegar el código del `Playground`. Ya realizados los cambios presionamos el botón `Commit changes`:

![commit](./images/commit.png)

Nos aparecerá un menú de confirmación donde nuevamente presionamos el botón `Commit changes`:

![commit2](./images/commit2.png)