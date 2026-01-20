import { AppErrors } from "../../domain/enums/errors.enum";
import { toaster } from "../toaster";

export function handleCommandError(error: any) {
  console.error(error);

  let description = "Ocurrió un error no controlado, por favor comuniquese con un desarrollador.";

  if (error?.type === AppErrors.DatabaseError) {
    description = "Ha ocurrido un error obteniendo la información de la base de datos";
  } else if (error?.type === AppErrors.ConfigError) {
    description = "Error al crear archivos de configuración, por favor comuniquese con un desarrollador";
  } else if (error?.type === AppErrors.IoError) {
    description = "Error al gestionar archivos";
  }

  toaster.error({
    title: "Error",
    description,
  });
}
