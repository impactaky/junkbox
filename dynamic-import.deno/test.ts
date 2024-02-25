import { dynamicImport } from 'https://deno.land/x/import/mod.ts';


await dynamicImport(Deno.args[0], { force: true });
