---
name: json-canvas
description: Create visual diagrams, flowcharts, mind maps, and system architectures using the JSON Canvas format. Use when you need to visualize processes, relationships, decision trees, or any structured thinking as a spatial layout openable in Obsidian or compatible viewers.
---

# JSON Canvas

Create visual diagrams, flowcharts, and mind maps using the JSON Canvas format — a simple, open spec for spatial layouts.

---

## When to Use

Trigger this skill when you hear:
- "Make me a flowchart"
- "Create a diagram of..."
- "Map this out visually"
- "I need a mind map"
- "Visualize this process"
- "Draw the architecture for..."
- "Show me how these connect"
- "Create a .canvas file"
- "I need to see the relationships between..."

This skill handles any situation where a user wants to turn ideas, processes, or relationships into a visual diagram they can open in a canvas viewer.

---

## Context Gathering

Before building the canvas, understand what the user wants to see. Ask conversationally — don't overwhelm them.

### The Subject
1. **What do you want to visualize?** (A process? A system? Ideas and their connections? An org structure?)
2. **Can you walk me through the pieces?** (The key things that should appear as boxes or nodes)

### The Relationships
3. **How do these pieces connect?** (Sequential steps? Hierarchical? Many-to-many? Cause and effect?)
4. **Are there any groupings?** (Things that belong together — departments, phases, categories)

### The Purpose
5. **Who is this for?** (Yourself for thinking? A team for alignment? A presentation? Documentation?)
6. **What format works best?** (Flowchart? Mind map? Org chart? Free-form spatial layout?)

### The Details
7. **Should any nodes link to files or URLs?** (Connect nodes to real documents, pages, or resources)
8. **Are there any labels, colors, or emphasis you want?** (Highlight critical paths, color-code categories, label decision points)

You won't always need to ask all of these. If someone says "flowchart of our onboarding process," you have enough to start. Ask follow-ups as you build.

---

## Methodology

### Step 1: Understand What to Visualize

Listen to what the user describes and identify the type of diagram:

- **Flowchart** — a process with sequential steps, decisions, and outcomes
- **Mind map** — a central idea branching out into related topics and subtopics
- **Org chart** — a hierarchy of people, teams, or components
- **System diagram** — components of a system and how they interact
- **Free-form** — ideas arranged spatially without strict structure

If the user has existing content (a document, a list, notes in memory), read that first:
```
agent(resource: "memory", action: "search", query: "relevant topic")
```

Use what you find to build the canvas. Don't make the user repeat what they've already told you.

### Step 2: Identify Nodes

Break the subject into individual nodes. Each node is a box on the canvas. Determine the node type for each:

- **text** — a box containing markdown text (most common)
- **file** — a box that references a file in the user's workspace
- **link** — a box that points to a URL
- **group** — a container that holds other nodes together visually

For each node, note:
- A unique ID (short, descriptive — like `start`, `review`, `deploy`)
- The content (text, file path, or URL)
- The approximate position (you'll calculate x/y coordinates)
- The size (width and height — typically 250x60 for simple nodes, larger for more text)

### Step 3: Identify Edges

Edges are the lines connecting nodes. For each connection, determine:

- **From node** — where the edge starts (use the node ID)
- **To node** — where the edge ends (use the node ID)
- **Direction** — which end gets the arrow (fromSide/toSide: top, right, bottom, left)
- **Label** — optional text on the edge ("yes", "no", "if approved", "then")
- **Color** — optional color to highlight certain paths (hex color code)

Common edge patterns:
- Flowcharts: top-to-bottom or left-to-right, arrows on the "to" end
- Mind maps: center outward, no arrows or arrows on outer ends
- Org charts: top-to-bottom, arrows pointing down
- System diagrams: bidirectional or labeled with data flow names

### Step 4: Calculate Layout

Position nodes so the diagram is readable. Use these spacing guidelines:

**Flowcharts (top-to-bottom):**
- Start at x:0, y:0 for the first node
- Space nodes 150px apart vertically
- Center parallel paths horizontally with 300px gaps
- Decision branches go left and right from the decision node

**Mind maps (center outward):**
- Central node at x:400, y:400
- First-level branches spaced evenly around the center (300px out)
- Second-level branches 250px further out from their parent
- Spread branches to avoid overlap

**Org charts (top-to-bottom):**
- Root node centered at top
- Each level 200px below the previous
- Sibling nodes spaced 300px apart horizontally

**Free-form:**
- Group related nodes close together
- Leave clear space between groups
- Keep the overall layout balanced

### Step 5: Generate the JSON Canvas

Build the canvas file following the JSON Canvas spec:

```json
{
  "nodes": [
    {
      "id": "node-1",
      "type": "text",
      "x": 0,
      "y": 0,
      "width": 250,
      "height": 60,
      "text": "Start Here"
    },
    {
      "id": "node-2",
      "type": "text",
      "x": 0,
      "y": 200,
      "width": 250,
      "height": 60,
      "text": "Next Step"
    },
    {
      "id": "group-1",
      "type": "group",
      "x": -25,
      "y": -25,
      "width": 600,
      "height": 400,
      "label": "Phase 1"
    }
  ],
  "edges": [
    {
      "id": "edge-1",
      "fromNode": "node-1",
      "fromSide": "bottom",
      "toNode": "node-2",
      "toSide": "top",
      "label": "then"
    }
  ]
}
```

**Node type reference:**

| Type | Required Fields | Purpose |
|------|----------------|---------|
| text | id, type, x, y, width, height, text | A box with markdown content |
| file | id, type, x, y, width, height, file | A box referencing a file path |
| link | id, type, x, y, width, height, url | A box pointing to a URL |
| group | id, type, x, y, width, height | A container grouping other nodes |

**Optional fields for any node:** color (hex like `"#FF0000"` or preset `"1"` through `"6"`)

**Edge fields:**

| Field | Required | Description |
|-------|----------|-------------|
| id | Yes | Unique edge identifier |
| fromNode | Yes | ID of the source node |
| toNode | Yes | ID of the target node |
| fromSide | No | Side of source node: top, right, bottom, left |
| toSide | No | Side of target node: top, right, bottom, left |
| label | No | Text label on the edge |
| color | No | Hex color or preset number |

### Step 6: Save the Canvas File

Write the JSON to a `.canvas` file:

Ask the user where they want the file saved, or suggest a reasonable name based on the content:

```
write_file(path: "process-flow.canvas", content: [the JSON])
```

Canvas files can be opened in Obsidian and other tools that support the JSON Canvas spec.

If the user wants the canvas stored in memory instead:
```
agent(resource: "memory", action: "store", key: "canvas/[descriptive-name]", value: "The full JSON Canvas content", layer: "working")
```

### Step 7: Describe What You Built

Tell the user what the canvas contains:
- How many nodes and edges
- The overall structure (flowchart with 3 decision points, mind map with 5 branches, etc.)
- Where the file was saved
- How to open it (Obsidian, or any JSON Canvas-compatible viewer)

Then ask: *"Want me to adjust anything? Add nodes, change the layout, or color-code certain paths?"*

---

## Output Format

The canvas file follows the JSON Canvas specification:

```json
{
  "nodes": [
    {
      "id": "unique-id",
      "type": "text",
      "x": 0,
      "y": 0,
      "width": 250,
      "height": 60,
      "text": "# Node Title\n\nNode content in markdown"
    }
  ],
  "edges": [
    {
      "id": "unique-edge-id",
      "fromNode": "source-node-id",
      "fromSide": "bottom",
      "toNode": "target-node-id",
      "toSide": "top"
    }
  ]
}
```

When presenting the canvas in chat (before saving), show a text summary:

```
Canvas: [Name]
Nodes: [count] ([breakdown by type])
Edges: [count]
Layout: [flowchart / mind map / org chart / free-form]

Structure:
  Start -> Step 1 -> Decision
    Yes -> Step 2a -> End
    No -> Step 2b -> Review -> Decision
```

---

## Quality Checks

Before delivering the canvas, verify:

- [ ] **Valid JSON** — the file parses without errors
- [ ] **All edges reference existing nodes** — no edges pointing to IDs that don't exist
- [ ] **No overlapping nodes** — every node has enough space so text is readable
- [ ] **Logical flow** — the diagram reads naturally in the expected direction
- [ ] **Groups contain their children** — group x/y/width/height actually encloses the grouped nodes
- [ ] **Labels are clear** — edge labels and node text are concise and meaningful
- [ ] **Consistent sizing** — similar nodes have similar dimensions
- [ ] **No orphan nodes** — every node connects to at least one edge (unless intentionally standalone)

If any check fails, fix the issue before saving the file.

---

## Examples

### Example 1: Simple Flowchart

**User says:** "Make a flowchart for our content approval process."

**You do:**
1. Ask: "Walk me through the steps — what happens from when someone writes content to when it goes live?"
2. User: "Writer drafts it, editor reviews it, if approved it goes to design, if not it goes back to the writer. After design, it gets a final review then publishes."
3. Identify nodes: Draft, Editor Review, Design, Final Review, Publish, and a decision point
4. Map the edges including the "approved/rejected" decision
5. Layout top-to-bottom with the rejection loop going left
6. Generate the JSON Canvas:

```json
{
  "nodes": [
    {"id": "draft", "type": "text", "x": 100, "y": 0, "width": 250, "height": 60, "text": "Writer Drafts Content"},
    {"id": "review", "type": "text", "x": 100, "y": 150, "width": 250, "height": 60, "text": "Editor Reviews"},
    {"id": "design", "type": "text", "x": 100, "y": 300, "width": 250, "height": 60, "text": "Design Pass"},
    {"id": "final", "type": "text", "x": 100, "y": 450, "width": 250, "height": 60, "text": "Final Review"},
    {"id": "publish", "type": "text", "x": 100, "y": 600, "width": 250, "height": 60, "text": "Publish"}
  ],
  "edges": [
    {"id": "e1", "fromNode": "draft", "fromSide": "bottom", "toNode": "review", "toSide": "top"},
    {"id": "e2", "fromNode": "review", "fromSide": "bottom", "toNode": "design", "toSide": "top", "label": "Approved"},
    {"id": "e3", "fromNode": "review", "fromSide": "left", "toNode": "draft", "toSide": "left", "label": "Rejected"},
    {"id": "e4", "fromNode": "design", "fromSide": "bottom", "toNode": "final", "toSide": "top"},
    {"id": "e5", "fromNode": "final", "fromSide": "bottom", "toNode": "publish", "toSide": "top"}
  ]
}
```

7. Save as `content-approval.canvas`
8. Say: *"Created a flowchart with 5 steps and a rejection loop. Saved as content-approval.canvas — open it in Obsidian to see the visual layout."*

### Example 2: Mind Map

**User says:** "Help me brainstorm features for our mobile app. Mind map it."

**You do:**
1. Ask: "What's the app? And what are the main categories of features you're thinking about?"
2. User: "It's a fitness app. Main areas: workouts, nutrition, social, and tracking."
3. Place "Fitness App" in the center
4. Branch out to the 4 main categories
5. Ask: "Any specific features under each?" and add sub-branches
6. Generate the canvas with the central node and branches radiating outward
7. Save as `fitness-app-features.canvas`
8. Ask: *"Want to add more branches or drill deeper into any of these categories?"*

### Example 3: System Architecture

**User says:** "Diagram our tech stack — frontend, backend, database, and third-party services."

**You do:**
1. Ask: "Walk me through each layer. What's in your frontend? Backend? What databases? Which third-party services?"
2. User gives the details
3. Create groups for each layer (Frontend, Backend, Data, External Services)
4. Place component nodes inside each group
5. Draw edges showing data flow between components
6. Color-code by layer
7. Save as `tech-stack.canvas`
8. Say: *"Built a system diagram with 4 layers and 12 components. The edges show how data flows between them."*

### Example 4: Decision Tree

**User says:** "I need a decision tree for our customer support escalation process."

**You do:**
1. Ask: "What's the first question the support person asks? And what are the possible paths from there?"
2. Map out each decision point and its branches
3. Use edge labels for "yes/no" or specific conditions
4. Color decision nodes differently from action nodes
5. Layout with decisions branching left/right and actions flowing down
6. Save as `support-escalation.canvas`
7. Say: *"Created a decision tree with 4 decision points and 7 possible outcomes. Each path is labeled with the condition that triggers it."*

---

## Related Skills

- **brainstorming** — generate ideas first, then visualize them as a mind map canvas
- **spec-writing** — diagram the system architecture described in a spec
- **deep-research** — visualize research findings as connected concept maps
- **competitor-alternatives** — map the competitive landscape visually
- **launch-strategy** — create flowcharts for launch timelines and dependencies
- **content-calendar** — visualize content plans as spatial layouts

**This skill turns any structured thinking into something visual.** Use it after brainstorming, planning, or research to make ideas tangible.
