import { invoke } from "@tauri-apps/api/core";
import { Response } from "@/utils/shared";
import { toast } from "vue-sonner";

// call is used to handle the results returned by wails.
export async function call<P, R>(method: string, data?: P): Promise<R | undefined> {
  try {
    const result = await invoke<Response<R>>(method, { req: data })
      .then((resp) => {
        if (resp.code !== 0) {
          console.log(resp.message);
        }

        return resp.result;
      })
      .catch((err: Response<unknown>) => {
        toast.error(err?.message ?? "Internal Error");
      });

    return result ?? undefined;
  } catch (err) {
    console.log(err);

    return undefined;
  }
}
