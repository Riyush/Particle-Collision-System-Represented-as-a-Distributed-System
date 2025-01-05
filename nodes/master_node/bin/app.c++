import master_node;
#include <iostream>
#include <vector>

int main()
{
	MasterNode master("127.0.0.1", "127.0.0.1:9090");

	master.allocate_worker_nodes();
	
}