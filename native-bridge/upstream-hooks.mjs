import { createHash } from "node:crypto";
export const UPSTREAM_COMMIT = "e62b270402b688a39a864007b9c0a02711b9573e";
export function patchSaveModule(source) {
  source = source.replace(/\r\n/g, "\n");
  if (
    createHash("sha256").update(source).digest("hex") !==
    "fadc8664fe1306804a10863b8b805066e8a775976fbe8dd5474a5206dc5a9234"
  )
    throw Error(
      "Neurite saver differs from the pinned baseline; no patch applied.",
    );
  const exporter = `    async nativeExport(){
        const meta = this.#selectedGraph || this.#makeMetaForTitle('HQ workspace');
        const data = await this.#saver.nativeSnapshot(meta);
        await this.#stored.saveMetaAndData(meta, data);
        if (!this.#graphs.includes(meta)) this.#graphs.push(meta);
        this.#setSelectedGraph(meta);
        return new GraphExporter(meta, this.#stored).export();
    }
    async nativeImport(file){
        const importer = new GraphImporter();
        await importer.import(file);
        if (!importer.data) throw new Error('Not a complete Neurite export. Current graph retained.');
        await this.#loadAndSave(importer, file.name);
        await this.#updateGraphs();
    }
`;
  return source
    .replace(
      "return stored.forEachMetaAndGraphId(this.#processMeta)",
      "return this.nativeReady = stored.forEachMetaAndGraphId(this.#processMeta)",
    )
    .replace(
      "mom.#stored.dataForMeta(meta).then(mom.#loadGraph.bind(mom));",
      "return mom.#stored.dataForMeta(meta).then(mom.#loadGraph.bind(mom));",
    )
    .replace("    #clearGraph(){", exporter + "    #clearGraph(){")
    .replace(
      "        saveWithTitle(title){",
      "        nativeSnapshot(meta){ return this.#makeSaveData(meta) }\n        saveWithTitle(title){",
    );
}
