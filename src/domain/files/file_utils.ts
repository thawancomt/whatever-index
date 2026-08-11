import { RemixiconComponentType, RiFileCodeLine, RiFileLine, RiFilePdf2Line } from "@remixicon/react"



export interface ExtensionType {
    type: ExtensionDomains[],
    icon: RemixiconComponentType
}

export enum ExtensionDomains {
    Textable = "Textable",
    Code = "Code",
    Document = "Document",
    unknown = "unknown"
}

const ExtensionMapper = {
    "txt": {
        type: [ExtensionDomains.Textable],
        icon: RiFileLine
    },
    "md": {
        type: [ExtensionDomains.Textable],
        icon: RiFileLine
    },

    // Code
    "json": {
        type: [ExtensionDomains.Code],
        icon: RiFileCodeLine
    },
    "js": {
        type: [ExtensionDomains.Code],
        icon: RiFileCodeLine
    },
    "ts": {
        type: [ExtensionDomains.Code],
        icon: RiFileCodeLine
    },

    // Documents

    "pdf": {
        type: [ExtensionDomains.Document, ExtensionDomains.Textable],
        icon: RiFilePdf2Line
    },
    "doc": {
        type: [ExtensionDomains.Document],
        icon: RiFileLine
    },
    "docx": {
        type: [ExtensionDomains.Document],
        icon: RiFileLine
    },
    "ppt": {
        type: [ExtensionDomains.Document],
        icon: RiFileLine
    },
    "pptx": {
        type: [ExtensionDomains.Document],
        icon: RiFileLine
    },
    "xls": {
        type: [ExtensionDomains.Document],
        icon: RiFileLine
    },
    "xlsx": {
        type: [ExtensionDomains.Document],
        icon: RiFileLine
    },

} as const

type KnownExtensions = keyof typeof ExtensionMapper


export const getExtension = (file: string): KnownExtensions => {
    return file.split('.').pop()?.toLowerCase() as KnownExtensions || undefined
}

export const getFileDomain = (file: string): typeof ExtensionMapper[keyof typeof ExtensionMapper] => {
    const extension = getExtension(file)
    return ExtensionMapper[extension] || {
        type: [ExtensionDomains.unknown],
        icon: RiFileLine
    }
}
