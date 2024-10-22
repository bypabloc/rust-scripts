Para poder ejecutar tu script de Rust desde cualquier carpeta sin necesidad de estar en el directorio del proyecto, puedes compilarlo y luego agregar el binario compilado a tu PATH. Aquí te explico los pasos para hacerlo:

1. **Compila el proyecto en modo release**:
   - Esto generará un binario optimizado que puedes usar desde cualquier parte del sistema.
   ```sh
   cargo build --release
   ```

2. **Encuentra el binario generado**:
   - El binario se encuentra en `target/release` dentro del directorio de tu proyecto.
   ```sh
   ls target/release/
   ```

3. **Agrega el binario a tu PATH**:
   - Puedes copiar el binario a una ubicación que ya esté en tu PATH, o puedes agregar el directorio `target/release` a tu PATH.
   
   Para copiar el binario (por ejemplo, `rust-scripts`):
   ```sh
   sudo cp target/release/rust-scripts /usr/local/bin/
   ```

   Alternativamente, puedes agregar `target/release` a tu PATH. Edita tu archivo `~/.bashrc` o `~/.zshrc` (dependiendo de la shell que uses) y añade la siguiente línea:
   ```sh
   export PATH=$PATH:/home/tu_usuario/projects/bypabloc/rust-scripts/target/release
   ```

   Luego, recarga el archivo de configuración de tu shell:
   ```sh
   source ~/.bashrc  # o ~/.zshrc
   ```

4. **Ejecuta el script desde cualquier ubicación**:
   - Ahora puedes ejecutar el script desde cualquier carpeta usando el comando del binario y pasando el argumento `--path`.

   ```sh
   rust-scripts --path="/home/bypablo/projects/destacame/easy-pay/executive_simulator/models"
   ```

### Resumen de Comandos:
1. Compila en modo release:
   ```sh
   cargo build --release
   ```

2. Copia el binario al PATH:
   ```sh
   sudo cp target/release/rust-scripts /usr/local/bin/
   ```

   O agrega el directorio `target/release` al PATH (y recarga la configuración de la shell):
   ```sh
   export PATH=$PATH:/home/tu_usuario/projects/bypabloc/rust-scripts/target/release
   source ~/.bashrc  # o ~/.zshrc
   ```

3. Ejecuta el script desde cualquier carpeta:
   ```sh
   rust-scripts --path="/home/bypablo/projects/destacame/easy-pay/executive_simulator/models"
   ```

Con estos pasos, tu script de Rust debería poder ejecutarse desde cualquier directorio en tu sistema.

### Usando el nuevo script para compilar y ejecutar el proyecto

Hemos agregado un nuevo script para automatizar la compilación del proyecto de Rust y proporcionar un comando de copia y pega para la ejecución. Aquí te explicamos cómo usarlo:

1. **Ejecuta el script de compilación y ejecución**:
   - Esto compilará el proyecto en modo release y generará un comando para ejecutar el binario compilado.
   ```sh
   ./scripts/build_and_run.sh
   ```

2. **Copia y pega el comando generado**:
   - El script imprimirá un comando en la consola que puedes copiar y pegar para ejecutar el binario compilado.
   ```sh
   To run the compiled binary, use the following command:
   ./target/release/rust-scripts
   ```

Con este nuevo script, puedes compilar y ejecutar fácilmente el proyecto de Rust sin necesidad de recordar los comandos manuales.
