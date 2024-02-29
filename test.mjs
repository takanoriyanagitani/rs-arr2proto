import { readFile } from "node:fs/promises";

(() => {
  return Promise.resolve("./rs_arr2proto.wasm")
    .then((name) => readFile(name))
    .then((buf) => WebAssembly.instantiate(buf))
    .then((pair) => {
      const {
        //module,
        instance,
      } = pair || {};
      const {
        memory,
        f64height,
        f64width,
        f64data_offset,
        f64data_count,
        f64resize,
        f64sum,
        f64serialize_init,
        f64serialize,
        f64serialized_offset,
        f64clear_decoded,
        f64decode,
      } = instance?.exports || {};
      const width = 1024;
      const height = 768;
      const count = width * height;
      console.info(`ser len:  ${f64serialize()}`);
      console.info(`sum nan:  ${f64sum()}`);
      console.info(`capacity: ${f64resize(width, height, 4.2)}`);
      console.info(`height:   ${f64height()}`);
      console.info(`width:    ${f64width()}`);
      console.info(`data cnt: ${f64data_count()}`);
      console.info(`dat ofst: ${f64data_offset()}`);
      const ser_init_start = Date.now();
      console.info(`ser len:  ${f64serialize_init()}`);
      console.info(`ser init time: ${Date.now() - ser_init_start}`);
      console.info(`ser ofst: ${f64serialized_offset()}`);
      const f64arr = new Float64Array(
        memory?.buffer,
        f64data_offset(),
        f64data_count(),
      );
      const ser_start = Date.now();
      console.info(`ser len:  ${f64serialize()}`);
      console.info(`ser time: ${Date.now() - ser_start}`);
      console.info(`sum:      ${f64sum()}`);
      console.info(f64arr);
      console.info(`ser ofst: ${f64serialized_offset()}`);
      console.info(`cleared:  ${f64clear_decoded()}`);
      console.info(`data cnt: ${f64data_count()}`);
      console.info(`decoded:  ${f64decode(count)}`);
      console.info(`data cnt: ${f64data_count()}`);
      return {
        memory,
        f64height,
        f64width,
      };
    })
    .catch(console.warn);
})();
