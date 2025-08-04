export class Violation {
    property: string;
    type: string;
    constructor({ property, type }: { property: string; type: string }) {
        this.property = property;
        this.type = type;
    }
}

export class ProblemDetails {
    type: string;
    title: string;
    status: number;
    detail: string;
    violations?: Violation[];

    constructor({ type, title, status, detail, violations }: {
        type: string;
        title: string;
        status: number;
        detail: string;
        violations?: Violation[];
    }) {
        this.type = type;
        this.title = title;
        this.status = status;
        this.detail = detail;
        if (violations) this.violations = violations;
    }
} 