package leetcode

type Node struct {
	Val       int
	Neighbors []*Node
}

func cloneGraph(node *Node) *Node {
	return dfs(node, map[*Node]*Node{})
}

func dfs(node *Node, lookup map[*Node]*Node) *Node {
	if node == nil {
		return nil
	}
	if clonedNode, ok := lookup[node]; ok {
		return clonedNode
	}
	newNode := &Node{
		Val: node.Val,
	}
	lookup[node] = newNode
	for _, neighbor := range node.Neighbors {
		newNode.Neighbors = append(newNode.Neighbors, dfs(neighbor, lookup))
	}
	return newNode
}
