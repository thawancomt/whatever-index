import { ExtensionDomains, getFileDomain } from "@/domain/files/file_utils";

export default function mapFilesByDomain(filePaths: string[]) {

    const mappedByDomain: Partial<Record<ExtensionDomains, string[]>> = {}

    for (const file of filePaths) {
        const { type } = getFileDomain(file)

        for (const domain of type) {
            if (!mappedByDomain[domain]) {
                mappedByDomain[domain] = []
            }
            mappedByDomain[domain].push(file)
        }
    }

    return mappedByDomain
}