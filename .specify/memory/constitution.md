<!--
Sync Impact Report:
- Version: [CONSTITUTION_VERSION] -> 1.0.0
- Modified Principles:
    - [PRINCIPLE_1_NAME] -> I. Clean Code & Standards
    - [PRINCIPLE_2_NAME] -> II. Test-First Methodology
    - [PRINCIPLE_3_NAME] -> III. Consistent User Experience
    - [PRINCIPLE_4_NAME] -> IV. Modular Architecture
- Added Sections:
    - Technology & Architecture
    - Development Workflow
- Templates requiring updates:
    - ✅ .specify/templates/plan.md (Aligned)
    - ✅ .specify/templates/spec.md (Aligned)
    - ✅ .specify/templates/tasks.md (Aligned)
-->

# GeminiSpec Constitution
<!-- Example: Spec Constitution, TaskFlow Constitution, etc. -->

## Core Principles

### I. Clean Code & Standards
<!-- Example: I. Library-First -->
Code must be clean, idiomatic, and rigorously adhere to project conventions. Automatic linting and formatting tools must be employed to enforce consistency. Comments should be used sparingly and focus on the "why" of complex logic, not the "what". Technical debt should be minimized by favoring simple, readable solutions over complex ones.

### II. Test-First Methodology
<!-- Example: II. CLI Interface -->
Testing is not an afterthought; it is a design tool. Tests MUST be written before implementation (TDD) whenever feasible. New features require unit tests for logic and integration tests for workflows. The "Red-Green-Refactor" cycle is the standard operating procedure. A feature is not complete until its corresponding tests pass.

### III. Consistent User Experience
<!-- Example: III. Test-First (NON-NEGOTIABLE) -->
All user interactions, whether via CLI or other interfaces, must be consistent, intuitive, and robust. Error messages must be actionable and helpful, not just descriptive. Output formats (e.g., JSON vs. Text) must remain consistent across commands to ensure composability and ease of automation.

### IV. Modular Architecture
<!-- Example: IV. Integration Testing -->
The system must be designed as a collection of modular, independent components. Features should be specified as independent "User Stories" that can be developed, tested, and delivered separately. Dependencies between modules must be explicit and minimized to prevent tight coupling and ensure maintainability.

## Technology & Architecture
<!-- Example: Additional Constraints, Security Requirements, Performance Standards, etc. -->

Adherence to the technology stack defined in `plan.md` is mandatory. Introducing new languages, frameworks, or heavy dependencies requires explicit justification and amendment of the plan. Prefer standard libraries and established patterns over experimental or obscure solutions unless they provide a critical advantage.

## Development Workflow
<!-- Example: Development Workflow, Review Process, Quality Gates, etc. -->

Development must follow the defined Speckit lifecycle: Specify -> Plan -> Tasks -> Implement. Prerequisite checks and quality gates (checklists) defined in the templates must be respected. No stage should be skipped; ensuring the specification and plan are solid before coding prevents costly rework.

## Governance
<!-- Example: Constitution supersedes all other practices; Amendments require documentation, approval, migration plan -->

This constitution is the supreme guide for the project. Its principles are non-negotiable requirements for all contributions. Amendments to this document require a formal version bump, justification, and a review of the impact on dependent templates.

**Version**: 1.0.0 | **Ratified**: 2025-12-08 | **Last Amended**: 2025-12-08
<!-- Example: Version: 2.1.1 | Ratified: 2025-06-13 | Last Amended: 2025-07-16 -->