import { Scenario } from "./Scenario";

export class Revision {
    version!: string;
    dependencies: string[] = [];
    scenarios: Scenario[] = [];
    downloaded!: boolean;
}