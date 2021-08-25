#include <iostream>
using namespace std;
 
// linked list node
struct Node
{
    int val;
    Node *next;
    Node() : val(0), next(nullptr) {}
    Node(int x) : val(x), next(nullptr) {}
    Node(int x, Node *next) : val(x), next(next) {}
};

void Play(Node* head, Node** cups, int count, int length)
{
    for (int i = 0; i < count; i++)
    {
        Node* pickup[3];
        Node* node = head;
        for (int j = 0; j < 3; j++)
        {
            node = node->next;
            pickup[j] = node;
            cups[node->val-1] = nullptr;
        }
        head->next = node->next;

        for (int j = 1;; j++)
        {
            int cupIndex = (head->val + length - j) % length;
            Node* cup = cups[cupIndex];
            if (cup != nullptr)
            {
                Node* nextCup = cup->next;
                cup->next = pickup[0];
                pickup[2]->next = nextCup;

                for (int k = 0; k < 3; k++)
                {
                    cups[pickup[k]->val-1] = pickup[k];
                }
                
                break;
            }
        }
        
        head = head->next;
    }
}

void PartOne(int input[])
{
    int length = 9;

    Node* cups[length];
    Node* startNode = new Node(input[0]);
    Node* previousNode = startNode;
    cups[input[0]-1] = startNode;
    for (int i = 1; i < length; i++)
    {
        Node* node = new Node(input[i]);
        cups[input[i]-1] = node;
        previousNode->next = node;
    }
    previousNode->next = startNode;

    Play(startNode, cups, 100, length);

    Node* cup = cups[0];
    cout << "Part 1: ";
    for (int i = 0; i < length-1; i++)
    {
        cup = cup->next;
        cout << cup->val;
    }
}

int main()
{
    // example:
    char puzzleInput[] = "389125467";

    // puzzle input:
    //char puzzleInput[] = "459672813";

    int length = 9;
    int input[length] = {0, 0, 0, 0, 0, 0, 0, 0, 0};
    for (int i = 0; i < length; i++)
    {
        input[i] = puzzleInput[i] - '0';
    }

    PartOne(input);
}
