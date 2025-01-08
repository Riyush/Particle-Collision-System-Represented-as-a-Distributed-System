#include <iostream>
#include <vector>
#include <string>

import network;
import master_node;

//This script needs to encapsulate all logic of starting the application
int main()
{
	std::string master_tcp = get_new_port();
	std::string master_udp = get_new_port();

	MasterNode master(master_tcp, master_udp);
	
	master.allocate_worker_nodes();
	
}