import master_node;
#include <iostream>
#include <vector>

//This script needs to encapsulate all logic of starting the application
int main()
{
	std::cout << "HELLO";
	MasterNode master("127.0.0.1", "127.0.0.1:9090");

	master.allocate_worker_nodes();
	
}